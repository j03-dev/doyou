from oxapy import (
    HttpServer,
    Request,
    Cors,
    Router,
    FileStreaming,
    get,
    exceptions,
)
from googleapiclient.discovery import build

import dotenv
import os
import yt_dlp
import time
import logging

dotenv.load_dotenv()


GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
MEDIA_DIR = "media"

youtube = build("youtube", "v3", developerKey=GOOGLE_API_KEY)


def search_youtube(r: Request):
    time.sleep(3)
    q = r.query()
    query_searh = q.get("q")

    if not query_searh:
        raise exceptions.BadRequestError("The 'q' query is not found!")

    try:
        request = youtube.search().list(
            part="snippet",
            q=query_searh,
            type="video",
            maxResults=10,
        )
        response = request.execute()
        return response
    except Exception as e:
        raise exceptions.InternalError(f"Verify your internet connection: {e}")


def download(r: Request):
    q = r.query()
    video_id = q.get("id")

    if not video_id:
        raise exceptions.BadRequestError("The 'id' query is not found!")

    path_part = f"{MEDIA_DIR}/{video_id}"

    if os.path.exists(f"{path_part}.mp3"):
        return {"video_id": video_id}

    url = f"https://www.youtube.com/watch?v={video_id}"
    ydl_opts = {
        "format": "bestaudio/best",
        "postprocessors": [
            {
                "key": "FFmpegExtractAudio",
                "preferredcodec": "mp3",
                "preferredquality": "192",
            }
        ],
        "outtmpl": f"{path_part}.%(ext)s",
        "noplaylist": True,
    }

    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        ydl.download([url])
    return {"video_id": video_id}


def listen(r: Request):
    q = r.query()
    video_id = q.get("id")
    assert video_id, "Video not found"
    path = f"{MEDIA_DIR}/{video_id}.mp3"
    return FileStreaming(path, content_type="audio/mpeg")


def log(r: Request, next, **kwargs):
    logging.log(1000, f"{r.method} {r.uri}")
    return next(r, **kwargs)


def main():
    (
        HttpServer(("0.0.0.0", 5555))
        .cors(Cors())
        .attach(
            Router()
            .middleware(log)
            .route(get("/health", lambda _r: "Good!"))
            .route(get("/api/v1/search", search_youtube))
            .route(get("/api/v1/download", download))
            .route(get("/api/v1/listen", listen))
        )
        .run()
    )


if __name__ == "__main__":
    main()

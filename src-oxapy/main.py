from oxapy import HttpServer, Request, Router, FileStreaming, get
from googleapiclient.discovery import build

import dotenv
import os
import yt_dlp

dotenv.load_dotenv()


GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
MEDIA_DIR = "media"

youtube = build("youtube", "v3", developerKey=GOOGLE_API_KEY)


def search_youtube(r: Request):
    q = r.query()
    query_searh = q["q"]
    assert query_searh, "must be pass query 'q'"
    request = youtube.search().list(
        part="snippet",
        q=query_searh,
        type="video",
        maxResults=10,
    )
    response = request.execute()
    return response


def download(r: Request):
    q = r.query()
    video_id = q.get("id")
    assert video_id, "Video not found"

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
        "outtmpl": f"{MEDIA_DIR}/{video_id}.%(ext)s",
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


def main():
    (
        HttpServer(("0.0.0.0", 5555))
        .attach(
            Router()
            .route(get("/health", lambda _r: "Good!"))
            .route(get("/api/v1/search", search_youtube))
            .route(get("/api/v1/download", download))
            .route(get("/api/v1/listen", listen))
        )
        .run()
    )


if __name__ == "__main__":
    main()

from oxapy import (
    HttpServer,
    Request,
    Cors,
    Router,
    FileStreaming,
    exceptions,
)
from googleapiclient.discovery import build

import contextlib
import dotenv
import os
import yt_dlp
import logging
import shutil
import tempfile

dotenv.load_dotenv()


GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
MEDIA_DIR = "media"
SECRET_FILE = "/etc/secrets/cookies.txt"
youtube = build("youtube", "v3", developerKey=GOOGLE_API_KEY)


@contextlib.contextmanager
def temp_cookie_file(original_path):
    if not os.path.exists(original_path):
        yield None
        return

    tmp_dir = tempfile.mkdtemp()
    try:
        temp_path = os.path.join(tmp_dir, "cookies.txt")
        shutil.copy(original_path, temp_path)
        yield temp_path
    finally:
        if tmp_dir:
            shutil.rmtree(tmp_dir)


def log(r: Request, next, **kwargs):
    logging.log(1000, f"{r.method} {r.uri}")
    return next(r, **kwargs)


router = Router()
router.middleware(log)


@router.get("/api/v1/search")
def search_youtube(r: Request):
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


@router.get("/api/v1/download")
def download(r: Request):
    q = r.query()
    video_id = q.get("id")

    if not video_id:
        raise exceptions.BadRequestError("The 'id' query is not found!")

    path_part = f"{MEDIA_DIR}/{video_id}"

    if os.path.exists(f"{path_part}.mp3"):
        return {"video_id": video_id}

    url = f"https://www.youtube.com/watch?v={video_id}"

    with temp_cookie_file(SECRET_FILE) as cookie_file:
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
        if cookie_file:
            ydl_opts["cookiefile"] = cookie_file

        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            ydl.download([url])
    return {"video_id": video_id}


@router.get("/api/v1/listen")
def listen(r: Request):
    q = r.query()
    video_id = q.get("id")
    assert video_id, "Video not found"
    path = f"{MEDIA_DIR}/{video_id}.mp3"
    return FileStreaming(path, content_type="audio/mpeg")


def main():
    server = HttpServer(("0.0.0.0", 8080))
    server.cors(Cors())
    server.attach(router)
    server.run()


if __name__ == "__main__":
    main()

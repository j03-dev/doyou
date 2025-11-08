from oxapy import (
    HttpServer,
    Request,
    Cors,
    Router,
    exceptions,
)
from googleapiclient.discovery import build

import contextlib
import dotenv
import os
import logging
import shutil
import tempfile
import subprocess
import json

dotenv.load_dotenv()


GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
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

    url = f"https://www.youtube.com/watch?v={video_id}"

    clean_env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}

    result = subprocess.run(
        [
            "yt-dlp",
            "-f",
            "bestaudio",
            "--no-playlist",
            "-j",  # Output video data as JSON to stdout
            url,
        ],
        capture_output=True,
        text=True,
        check=True,
        env=clean_env,
    )

    video_data = json.loads(result.stdout)

    final_url = video_data.get("url")

    headers = video_data.get("http_headers", {})
    user_agent = headers.get("User-Agent")

    return {
        "url": final_url,
        "required_user_agent": user_agent,  # Provide this for debugging/client use
    }


def main():
    server = HttpServer(("0.0.0.0", 8080))
    server.cors(Cors())
    server.attach(router)
    server.run()


if __name__ == "__main__":
    main()

from oxapy import (
    HttpServer,
    Request,
    Cors,
    Router,
    FileStreaming,
    get,
    post,
    exceptions,
    jwt,
    serializer,
)
from googleapiclient.discovery import build
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

from models import User, LikedSong

import dotenv
import os
import yt_dlp
import logging
import uuid

dotenv.load_dotenv()

TURSO_DB_AUTH_TOKEN = os.getenv("TURSO_DB_AUTH_TOKEN")
TURSO_DB_URL = os.getenv("TURSO_DB_URL")
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
MEDIA_DIR = "media"
ENGINE = create_engine(
    "sqlite+libsql:///local.db",
    connect_args={
        "auth_token": TURSO_DB_AUTH_TOKEN,
        "sync_url": TURSO_DB_URL,
    },
)
DB = sessionmaker(bind=ENGINE)
SECRET = os.getenv("SECRET")
JWT = jwt.Jwt(SECRET)

youtube = build("youtube", "v3", developerKey=GOOGLE_API_KEY)


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


def register(r: Request):
    new_user = User(id=str(uuid.uuid4()))
    r.db.add(new_user)
    r.db.commit()

    claims = {"sub": new_user.id, "exp": 3600 * 24 * 365}
    token = JWT.generate_token(claims)
    return {"token": token}


class LikedSongSerializer(serializer.Serializer):
    id = serializer.CharField()
    user_id = serializer.CharField()


def like(r: Request, video_id: str):
    liked_song = LikedSong(id=video_id, user_id=r.user)
    r.db.add(liked_song)
    r.db.commit()
    liked_song_serializer = LikedSongSerializer(instance=liked_song)
    return {"liked_song": liked_song_serializer.data}


def likes(r: Request, video_id: str):
    liked_songs = r.db.query(LikedSong).filter(User.id == r.user.id)
    liked_song_serializer = LikedSongSerializer(instance=liked_songs, many=True)
    return {"liked_songs": liked_song_serializer.data}


def db_session(r: Request, next, **kwargs):
    db = DB()
    try:
        r.db = db
        return next(r, **kwargs)
    finally:
        db.close()


def log(r: Request, next, **kwargs):
    logging.log(1000, f"{r.method} {r.uri}")
    return next(r, **kwargs)


def jwt_auth(r: Request, next, **kwargs):
    if token := r.headers.get("authorization", "").replace("Bearer ", ""):
        try:
            db = DB()
            claims = JWT.verify_token(token)
            user_id = claims["sub"]
            r.user = db.get(User, user_id)
            return next(r, **kwargs)
        except jwt.JwtError as e:
            return {"detail": str(e)}
        finally:
            db.close()
    raise exceptions.UnauthorizedError("token must be provide")


def main():
    (
        HttpServer(("0.0.0.0", 8080))
        .cors(Cors())
        .attach(
            Router()
            .middleware(log)
            .middleware(db_session)
            .route(get("/api/v1/search", search_youtube))
            .route(get("/api/v1/download", download))
            .route(get("/api/v1/listen", listen))
            .route(post("/api/v1/register", register))
        )
        .attach(
            Router()
            .middleware(log)
            .middleware(jwt_auth)
            .middleware(db_session)
            .route(post("/api/v1/like", like))
            .route(get("/api/v1/like", likes))
        )
        .run()
    )


if __name__ == "__main__":
    main()

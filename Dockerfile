FROM python:3.14-slim

WORKDIR /app

# RUN apt-get update && \
#     apt-get install -y ffmpeg && \
#     rm -rf /var/lib/apt/lists/*

RUN apt-get update \
 && apt-get install -y --no-install-recommends ffmpeg ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && ffmpeg -version \
 && ffprobe -version


# Ensure they're accessible on a "normal" PATH (optional but helps)
RUN ln -sf "$(which ffmpeg)" /usr/local/bin/ffmpeg \
 && ln -sf "$(which ffprobe)" /usr/local/bin/ffprobe

COPY ./src-oxapy .

RUN pip install .

EXPOSE 8080

CMD ["python", "main.py"]

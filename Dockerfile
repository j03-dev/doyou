FROM python:3.14-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ffmpeg && \
    rm -rf /var/lib/apt/lists/*

COPY ./src-oxapy .

RUN pip install .

EXPOSE 8080

CMD ["python", "main.py"]

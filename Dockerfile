FROM python:3.14-slim

WORKDIR /app

COPY ./src-oxapy .

RUN pip install .

EXPOSE 8080

CMD ["python", "main.py"]

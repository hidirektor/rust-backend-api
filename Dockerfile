FROM ubuntu:latest
LABEL authors="hidirektor"

ENTRYPOINT ["top", "-b"]
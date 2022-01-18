FROM --platform=linux/x86_64 rust:1.58-slim

RUN apt-get update
RUN apt-get install -y locales tmux
RUN locale-gen ja_JP.UTF-8
RUN localedef -f UTF-8 -i ja_JP ja_JP
ENV LANG ja_JP.UTF-8
ENV TZ Asia/Tokyo

WORKDIR /app
COPY . /app/
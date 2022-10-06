FROM rust:buster

RUN apt update
RUN apt upgrade -y
RUN apt install python3-pip vim -y

WORKDIR /home

COPY . .
RUN cargo build
FROM rust:latest

WORKDIR /usr/src/digital_cookbook

COPY . .

RUN cargo install --path .


EXPOSE 8080

CMD ["digital_cookbook"]
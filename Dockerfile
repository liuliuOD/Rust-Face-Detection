FROM rust:1.67

WORKDIR /app

RUN apt update -y && apt install -y libopencv-dev clang libclang-dev

COPY . .

RUN cargo build

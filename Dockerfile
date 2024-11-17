FROM clux/muslrust:stable

WORKDIR /app
COPY . .

RUN cargo build --release

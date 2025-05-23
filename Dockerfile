FROM rust:1.87 AS builder

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/dbms_proj_backend /usr/local/bin/dbms_proj_backend

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/dbms_proj_backend"]

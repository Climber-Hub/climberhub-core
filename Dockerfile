FROM rust:1.69-bullseye as builder
WORKDIR /app
COPY src /app/src
COPY Cargo.* /app
RUN cargo install --path .


FROM debian:bullseye-slim as runner
COPY --from=builder /usr/local/cargo/bin/climberhub-core /usr/local/bin/climberhub-core
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["climberhub-core"]
FROM clux/muslrust AS build
RUN mkdir /app
WORKDIR /app

COPY ./ ./
RUN cargo build --release

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/bot .
COPY --from=build /app/config.toml .
COPY --from=build /app/db.toml .
USER 1000
CMD ["./bot"]

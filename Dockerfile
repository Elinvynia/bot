FROM clux/muslrust AS build
RUN mkdir /app
WORKDIR /app

COPY ./ ./
RUN cargo build --release

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/bot .
USER 1000
CMD ["./bot"]

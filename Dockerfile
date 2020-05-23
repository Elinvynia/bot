FROM clux/muslrust AS build
WORKDIR /app

# Create blank project.
RUN USER=root cargo new bot

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /app/bot/

WORKDIR /app/bot/

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

# Now copy in the rest of the sources.
COPY src /app/bot/src/
COPY config.toml /app/bot/

# This is the actual build.
RUN cargo build --release


FROM scratch
COPY --from=build /app/bot/target/x86_64-unknown-linux-musl/release/bot .
COPY --from=build /app/bot/config.toml .
USER 1000
CMD ["./bot"]

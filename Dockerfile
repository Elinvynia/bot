FROM clux/muslrust AS build
WORKDIR /app

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock ./

# Create a dummy file.
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Build dependencies.
RUN cargo build --release
RUN rm target/x86_64-unknown-linux-musl/release/deps/bot*

# Now copy in the rest of the sources.
COPY src ./src
COPY config.toml ./

# This is the actual build.
RUN cargo build --release

#FROM scratch
#COPY --from=build /app/bot/target/x86_64-unknown-linux-musl/release/bot .
#COPY --from=build /app/bot/config.toml .
#USER 0
#CMD ["./bot"]

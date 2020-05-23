# Eli Bot

A Discord bot written using the Serenity framework in Rust. With a focus on proper logging and zero embeds, this general purpose bot was made to fill in the gap in the market. Everyone is encouraged to run their own.

## Manual setup
1. Open `config.toml`
2. Insert your Discord bot token
3. `cargo build` to get the executable
4. `cargo run` to run the bot

## Docker-compose setup
```
version: "3"

services:
  bot:
    image: "elinvynia/bot:latest"
    environment:
      - DISCORD_TOKEN=YourTokenHere
    volumes:
      - .path/to/local/db.sqlite3:/db.sqlite3
```

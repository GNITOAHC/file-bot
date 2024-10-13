# file-bot

A simple Axum web service, built on top of Discord APIs.

## Features

-   POST to `/discord` endpoint to upload a file
-   Returns a JSON response containing the status and the URL of the uploaded file.

For instance:

```bash
curl -F 'file=@./png.png' http://127.0.0.1:8000/discord
```

## Self Hosting via Shuttle

### Prerequisites

-   Discord bot token
-   Discord guild id
-   Discord channel name
-   Rust
-   cargo-shuttle

### Running the Service

1.  Clone the repo via `git clone git@github.com:GNITOAHC/file-bot.git`
2.  Setup `Secrets.toml`
3.  Either deploy the service via `shuttle deploy` or host it locally by running `shuttle run`

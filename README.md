# Highscore-rs

A simple highscore server written in Rust.
May be used for small games like [Ludum Dare games](https://ldjam.com/).

## Usage

Just run with docker, for example:
```bash
docker run --rm -p 8000:8000 -e ROCKET_LOG_LEVEL=normal -v highscore:/home/highscore/data ghcr.io/aligator/highscore-rs:latest
```

or with docker-compose:
```bash
docker-compose up -d
```


or compile it yourself:
```bash
ROCKET_LOG_LEVEL=normal cargo run --release
```

## API
Just see [http://localhost:8000/swagger-ui/](http://localhost:8000/swagger-ui/) after starting the server.

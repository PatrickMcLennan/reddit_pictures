# reddit_pictures

Rust script that checks a subreddit for new pictures and downloads any not in your $DIR_PATH, sending a report to a custom slack channel on each new download.

## How to run

1. Clone repo.
2. Create `.env` file from `.env.example`.

### With [Docker](https://www.docker.com/) (recommended)

```bash
docker build -t reddit_pictures .;
docker run -v $DIR_PATH:$DIR_PATH --env-file=.env reddit_pictures;
```

### With native [Rust](https://www.rust-lang.org/)

```bash
cargo build --release;
$DIR_PATH/target/release/reddit_pictures;
```

# reddit_pictures

Rust script that checks a subreddit for new pictures and downloads any not in your $DIR_PATH.

## How to run

1. Clone repo.
2. Create `.env` file from `.env.example`.


### With Rust
[Rust](https://www.rust-lang.org/):

```bash
cargo build --release;
$DIR_PATH/target/release/reddit_pictures
```

### With Docker
[Docker](https://www.docker.com/)

```bash
docker build -t reddit_pictures .;
docker run -v $DIR_PATH:$DIR_PATH --env-file=.env reddit_pictures;
```
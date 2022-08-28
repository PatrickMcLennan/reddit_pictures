FROM rust:1.63.0
COPY . /app
WORKDIR /app
RUN cargo build --release
CMD /app/target/release/reddit_pictures
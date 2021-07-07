FROM rust:1.53-slim

WORKDIR /src-root
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config wait-for-it

# install movine for database migrations
RUN cargo install movine

ENTRYPOINT ["wait-for-it", "db:5432", "--", "./scripts/run_dev.sh"]

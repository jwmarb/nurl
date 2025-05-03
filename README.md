nurl is a URL shortener service built with Rust and React. It's a final project for a Computer Science class taught at the University of Arizona and was made in less than 24 hours. It's a simple and effective way to shorten URLs and keep track of their usage.

note: nurl is not production-ready and still needs work on authentication since anyone can register an account and create an unlimited amount of shortened urls. this is purely for educational purposes only.

## Frontend

Install dependencies:

To run:

```sh
pnpm install
```

## Backend

No need to install dependencies--Rust will download and compile their binaries.

To run:

```sh
cargo run
```

## Deploying nurl

### Docker

Build the docker image via

```sh
docker compose -f docker-compose.prod.yaml build
```

Run the docker container via

```sh
docker compose -f docker-compose.prod.yaml up
```

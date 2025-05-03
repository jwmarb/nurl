FROM node:23-slim AS frontend

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
    
RUN corepack enable

COPY frontend /app
WORKDIR /app

RUN pnpm install && pnpm run build

FROM rust:1-slim-bookworm AS backend

ENV ENVIRONMENT="production" \
    PORT="8080" \
    HOST="0.0.0.0"

COPY backend /app
WORKDIR /app

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y
RUN mkdir -p /app

COPY --from=backend /app/target/release/backend /app
COPY --from=frontend /app/build /app/dist

WORKDIR /app

CMD [ "./backend" ]
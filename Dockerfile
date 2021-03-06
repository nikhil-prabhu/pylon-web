# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Build Rust backend.
# -----------------------------------------------------------------------------

FROM rust:1.60-bullseye AS backend
WORKDIR /backend
COPY ./backend .

RUN cargo build --release

# -----------------------------------------------------------------------------
# Build static React frontend.
# -----------------------------------------------------------------------------

FROM node:16.15-alpine AS frontend
WORKDIR /frontend
COPY ./frontend .

RUN npm install && npm run build

# -----------------------------------------------------------------------------
# Construct final app image.
# -----------------------------------------------------------------------------

FROM debian:bullseye-slim AS production
WORKDIR /app
COPY --from=backend /backend/target/release/pylon-web .
COPY --from=frontend /frontend/build ./static
ENV PYLON_STATIC_DIR=/app/static
ENV ROCKET_ADDRESS="0.0.0.0"
EXPOSE 8080
CMD ["./pylon-web"]

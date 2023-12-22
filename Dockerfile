FROM rust:1.73.0-bullseye as build-app
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM oven/bun:1 as build-assets
WORKDIR /app
COPY . /app
COPY package.json bun.lockb .
RUN bun install 
RUN bunx tailwindcss -i src/assets/styles.css -o public/styles.css --minify

FROM gcr.io/distroless/cc
COPY --from=build-app /app/target/release/quitting-eurorack-v3-rs /
COPY --from=build-assets /app/public /public
CMD ["./quitting-eurorack-v3-rs"]

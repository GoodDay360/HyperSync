FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Bun builder stage
FROM oven/bun:1.2.23 AS bun-builder
WORKDIR /app
COPY . .
RUN bun install && bun run build


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY . .
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application

RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/local/bin
COPY --from=bun-builder /app/dist /usr/local/bin
COPY --from=builder /app/target/release/HyperSync /usr/local/bin
RUN ls -l /usr/local/bin
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/HyperSync"]
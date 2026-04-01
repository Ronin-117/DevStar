# Stage 1: Rust builder
FROM rust:1.82-slim AS rust-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/src-tauri
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch && rm -rf src
COPY src-tauri/ .
RUN cargo build --release && rm -rf src

# Stage 2: Node builder
FROM node:20-alpine AS node-builder

WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

# Stage 3: Final validation
FROM alpine:3.19

WORKDIR /app
COPY --from=node-builder /app/dist ./dist
COPY --from=rust-builder /app/src-tauri/target/release/projecttracker ./binary

RUN echo "Build successful — frontend dist and Rust binary produced"

CMD ["echo", "ProjectTracker build complete"]

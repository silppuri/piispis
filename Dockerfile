FROM node:22-slim

# Install Rust and wasm-pack
# Debian (not Alpine/musl) because wasm-pack downloads a glibc-linked wasm-bindgen-cli binary
RUN apt-get update && apt-get install -y --no-install-recommends curl build-essential ca-certificates && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install wasm-pack

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN wasm-pack build --out-dir pkg --out-name index --dev
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]
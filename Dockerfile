# build client js
FROM node:22 AS client
WORKDIR /src
# packages
COPY package.json .
COPY package-lock.json .
COPY client/package.json client/package.json
COPY client/package-lock.json client/package-lock.json
RUN npm run init
# src files
COPY client client
RUN npm run build

# build server rs
FROM rust:1.89-slim AS server
WORKDIR /src
# packages
COPY server/Cargo.toml .
COPY server/Cargo.lock .
# build packages only
RUN mkdir -v src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -Rvf src
# src files
COPY server/src src
RUN cargo build --release

# host
FROM rust:1.89-slim
WORKDIR /srv
ENV IS_RELEASE=1
# built files
COPY --from=client /src/dist dist
COPY --from=server /src/target/release/server bin/server
# exec
WORKDIR /srv/bin
CMD ["./server"]

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
FROM rust:1.95-slim AS server
WORKDIR /src
# packages
COPY server/Cargo.toml .
# build packages only
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs
# src files
COPY server/src ./src/
RUN touch src/main.rs
RUN cargo build --release

# host
FROM rust:1.95-slim
WORKDIR /srv
ENV IS_RELEASE=1
# built files
COPY --from=client /src/dist dist
COPY --from=server /src/target/release/server bin/server
# exec
WORKDIR /srv/bin
CMD ["./server"]

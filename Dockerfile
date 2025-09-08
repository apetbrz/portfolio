FROM node:22 AS client
WORKDIR /src
COPY package.json .
COPY package-lock.json .
COPY client/package.json client/package.json
COPY client/package-lock.json client/package-lock.json
RUN npm run init
COPY client client
RUN npm ll
RUN npm run build
RUN ls
RUN ls client

FROM rust:1.89-slim
WORKDIR /src
COPY --from=client /src/client/build dist
COPY server server
WORKDIR /src/server
ENV IS_RELEASE=1
RUN cargo build --release
CMD ["/src/server/target/release/server"]

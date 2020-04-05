FROM rust:1.40 AS build

WORKDIR /usr/src/api
COPY . .
RUN cargo build --release

FROM alpine:3.11

COPY --from=build /usr/local/cargo/bin/api /usr/local/bin/api

CMD /api
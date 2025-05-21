FROM rust:latest as build
WORKDIR /usr/src/FAGIA
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=build /usr/src/FAGIA/target/release/FAGIA /usr/local/bin/FAGIA
CMD ["FAGIA"]
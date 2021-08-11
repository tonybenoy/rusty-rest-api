FROM rust:buster as builder

RUN apt-get update
RUN apt-get install libpq-dev -y



COPY ./animal_insurance /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=builder /app/target/release/animal_insurance  /usr/local/bin/animal_insurance

CMD ["animal_insurance"]

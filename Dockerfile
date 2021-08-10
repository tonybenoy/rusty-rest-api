FROM rust:buster as builder
RUN apt-get update
COPY animal_insurance/ /app
WORKDIR /app
RUN cargo build --release
ENV db_user=${db_user}
ENV db_password=${db_password}
ENV db_url=${db_url}
ENV db_port=${db_port}
ENV db_name=${db_name}
ENV server_port=${server_port}
CMD ["./target/release/animal_insurance"]

# FROM debian:buster-slim as runner
# COPY --from=builder /usr/local/cargo/bin/animal_insurance /usr/local/bin/animal_insurance
# ENV db_user=${db_user}
# ENV db_password=${db_password}
# ENV db_url=${db_url}
# ENV db_port=${db_port}
# ENV db_name=${db_name}
# ENV server_port=${server_port}
# CMD ["/animal_insurance"]

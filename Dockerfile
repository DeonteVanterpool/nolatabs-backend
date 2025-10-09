FROM rust:latest as builder
WORKDIR /usr/src/nolatabs-backend
COPY . .
RUN cargo install --path .

FROM debian:stable
RUN apt-get update && rm -rf /var/lib/apt/lists/* # && apt-get install -y extra-runtime-dependencies 
COPY --from=builder /usr/local/cargo/bin/nolatabs-backend /usr/local/bin/nolatabs-backend
CMD ["nolatabs-backend"]


FROM rust:alpine as builder
WORKDIR /usr/src/nolatabs-backend
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk upgrade # && rm -rf /var/lib/apt/lists/* && apt-get install -y extra-runtime-dependencies 
COPY --from=builder /usr/src/nolatabs-backend/target/release/nolatabs-backend .
EXPOSE 3892
CMD ["nolatabs-backend"]


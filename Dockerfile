FROM rust:alpine as builder
WORKDIR /usr/src/nolatabs-backend
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
RUN apk update && apk upgrade # && rm -rf /var/lib/apt/lists/* && apt-get install -y extra-runtime-dependencies 
WORKDIR /app
COPY --from=builder /usr/src/nolatabs-backend/target/release/nolatabs-backend /app/
EXPOSE 3892
CMD ["./nolatabs-backend"]


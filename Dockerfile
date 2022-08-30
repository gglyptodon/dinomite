FROM rust:1.62.0 AS builder
WORKDIR /app
COPY . .
RUN cargo install wasm-pack && wasm-pack build --target web -d wasm
RUN rm -r src/

FROM python:3-alpine AS runtime
RUN adduser -D static
USER static
COPY --from=builder /app/ dinomite
WORKDIR dinomite 
EXPOSE 7000/tcp
CMD ["python", "-m", "http.server", "7000"]

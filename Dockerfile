FROM node:20.4.0-alpine3.18 as fe-builder
WORKDIR /app
COPY . .
RUN cd fe && \
    npm install -g pnpm && \
    pnpm install --frozen-lockfile && \
    pnpm run build

FROM rust:1.71.0-alpine3.18 as api-builder
WORKDIR /app
COPY . .
COPY --from=fe-builder /app/fe/build /app/fe/build
RUN apk add --no-cache musl-dev libpq-dev
RUN cargo build --release

FROM alpine:3.18
COPY --from=api-builder /app/target/release/slogin-be /usr/local/bin
CMD [ "slogin-be" ]

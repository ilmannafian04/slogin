FROM ekidd/rust-musl-builder:stable as be-builder
WORKDIR /app
COPY --chown=rust:rust . .
RUN cd be &&\
    cargo build --release

FROM node:alpine as fe-builder
WORKDIR /app
COPY . .
RUN cd fe && \
    npm install -g pnpm && \
    pnpm install --frozen-lockfile && \
    pnpm run build

FROM nginx:alpine
COPY --from=be-builder /app/be/target/x86_64-unknown-linux-musl/release/slogin-be /usr/local/bin
COPY --from=fe-builder /app/fe/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
RUN apk --no-cache add ca-certificates
CMD [ "/bin/sh", "-c", "nginx && slogin-be" ]


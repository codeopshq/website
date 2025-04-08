FROM rust:alpine as builder

RUN apk update && apk add --no-cache musl-dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

WORKDIR /app

COPY . .

RUN trunk build --release

FROM nginx:alpine

COPY nginx/default.conf /etc/nginx/conf.d/default.conf

# COPY dist /usr/share/nginx/html

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

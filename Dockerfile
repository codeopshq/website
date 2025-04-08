FROM rust:alpine as builder

RUN apk update && apk add --no-cache musl-dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

WORKDIR /app

# --- Define build arguments ---
ARG ARG_YOUTUBE_API_KEY
ARG ARG_YOUTUBE_API_URL
ARG ARG_YOUTUBE_CHANNEL_ID

# --- Set environment variables for the build process ---
ENV YOUTUBE_API_KEY=$ARG_YOUTUBE_API_KEY
ENV YOUTUBE_API_URL=$ARG_YOUTUBE_API_URL
ENV YOUTUBE_CHANNEL_ID=$ARG_YOUTUBE_CHANNEL_ID

COPY . .

RUN trunk build --release

FROM nginx:alpine

COPY nginx/default.conf /etc/nginx/conf.d/default.conf

# COPY dist /usr/share/nginx/html

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

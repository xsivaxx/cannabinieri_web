FROM rust:latest as builder
WORKDIR /usr/src/cannabinieri_web
COPY . .

# create release build
## ENV CARGO_HOME=/usr/local/cargo
RUN cargo install --path .


FROM debian:bookworm-slim
WORKDIR /usr/src/cannabinieri_web
COPY --from=builder /usr/local/cargo/bin/cannabinieri_web /usr/local/bin/cannabinieri_web
COPY --from=builder /usr/src/cannabinieri_web .
# run executable
CMD ["cannabinieri_web"] 
# expose port 5000
EXPOSE 5000


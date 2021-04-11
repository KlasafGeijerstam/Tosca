FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.51 as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release
ARG REDIRECT_URL="http://example.com"

FROM rust:1.51 as runtime
WORKDIR app
COPY --from=builder /app/target/release/toml_provider /usr/local/bin
COPY res/user_db.toml /

ENTRYPOINT ["toml_provider", "/user_db.toml"]
CMD ["9000"]

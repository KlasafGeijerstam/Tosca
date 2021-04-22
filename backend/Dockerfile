FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM rust:1.51 as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build

FROM rust:1.51 as runtime
WORKDIR app
COPY --from=builder /app/target/debug/dev_login /usr/local/bin/
COPY --from=builder /app/target/debug/seed_db /usr/local/bin/
COPY --from=builder /app/target/debug/toml_provider /usr/local/bin/
COPY --from=builder /app/target/debug/web_backend /usr/local/bin/

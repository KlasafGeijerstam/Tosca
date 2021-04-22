# Tosca test-data generator
This generator fills a tosca database with users, as well
as generating a toml-database suitable to be used with the
[toml-user-provider](../user/toml_provider)

## Generating data and filling a databse

* Set the environment variable `DATABASE_URL`, or pass the `--database_url` with
  the following format: `postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE`
* Run `cargo run -- output.toml`
    * The `output.toml` file can be loaded by the `toml-provider`

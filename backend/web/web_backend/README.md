# REST-API

All REST-operations must supply a session token as a http bearer token.

## Workspace

* GET `/workspaces`
* POST `/workspaces` [Super, Admin]
* GET `/workspaces/{workspace_id}`
* DELETE `workspaces/{workspace_id}` [Super, Creator]
* POST `/workspaces/{workspace_id}/moderators` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/moderators/{moderator_id}` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/whitelist/` [Super, Creator, Moderator]
* POST `/workspaces/{workspace_id}/whitelist` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/whitelist/{user_id}` [Super, Creator, Moderator]

## Starting the Tosca web-backend

Tosca web-backend requires three auxiliary services to be started before
it can function properly. These services are a *Login provider*, a *User provider*
and a *PostgreSQL* database. The web-backend takes a configuration file in TOML-format
as an argument.

### Login provider
For testing, the easiest setup is to use the `dev_login` login provider. The login
provider takes two arguments, the port to listen on and the URL to redirect logged
in users to. The login providers address and port must be configured in the web-backends
configuration file.


### User provider
The user provider provides the web-backend with user information. The `toml_provider` is
a user provider made specifically for testing, as it loads user and workspace information
from a TOML-file. The user provider takes one argument, the port to listen to. The user providers
host and port must be configured in the web-backends configuration file.


### PostgreSQL

Tosca uses a PostgreSQL database to store and retreive data. The database must first be initialized
by using `Diesel`. See [db\_connector](../db_connector/).

The database connection for the web-backend can be configured in two ways.

1. Within the `database` section of the configuration file.
2. By overriding the configuration file with the `--database` argument. This argument
   takes a connection-url of the following format: `postgres://{user}:{password}@{host}:{port}/{database}`


### Certification 

The script `generate_certificate` needs to be run before the backend is started the first
time to generate the https certificate.


### Sample startup
1. Start the database.
    * Enter the database connection information into the configuration file. OR
    * Create a connection url of the aforementioned format.
2. Start the login provider.
    * Go to the `dev_login` directory.
    * Run `cargo run -- 9000 https://{login_redirect}:{port}`
    * Enter the host of the login provider into the configuration file.
    * In this example, the login provider would listen on port 9000.
3. Start the user provider.
    * Go to the `toml_provider` directory.
    * run `cargo run -- 8000 res/user_db.toml`. This starts the user provider on port 8000, and loads the TOML-database `res/user_db.toml`.
    * Enter the host of the user provider into the configuration file.
4. Start the web-backend.
    * Go to the `web-backend` directory.
    * If you entered the database information into the configuration file:
        * Run `cargo run -- config.toml`, where `config.toml` is the configuration file.
    * Else (you created a connection-url for the database)
        * Run `cargo run -- config.toml --database {connection-url}`
5. You should now be able to access the REST-API on the configured port.


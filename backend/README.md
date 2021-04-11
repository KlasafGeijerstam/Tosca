## Setting up a development environment
The following dependencies must be installed to build and set up the backend
using Docker.

* `docker`
* `docker-compose`
* `rust` >= 1.50
* `diesel`

A development environment can be set up by running the following commands:
* `docker-compose build`
* `docker-compose up`

And then:
* `cd` to the `web/db_connector` directory.
* Move the `.env.example` file to `.env` (`mv .env.example .env`)
* Run the database migrations using: `diesel migration run`

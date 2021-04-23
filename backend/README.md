## Setting up a development environment
The following dependencies must be installed to build and set up the backend
using Docker.

* `docker`
* `docker-compose`

A development environment can be set up by running the following commands:
Copy the `.env.example` to `.env` and modify it if needed.

* `./setup.sh`
* `docker-compose build`
* `./seed_db.sh`
* `docker-compose up`

A development environment with a seeded database should now be up and running.

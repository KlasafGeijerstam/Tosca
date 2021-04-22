#!/bin/bash
if [ ! -f ".env" ]; then
    echo ".env file does not exist!"
    exit
fi

source .env
DB_CONTAINER=tosca_migration_db
export DATABASE_URL="postgresql://${PG_USER}:${PG_PASSWORD}@${PG_HOST}:${PG_PORT}/${PG_DB}"

echo 'Running migrations..'
sudo docker-compose run --rm database_migration
sudo docker-compose stop

sudo docker container rm $DB_CONTAINER
sudo docker-compose run -d --rm --service-ports --name $DB_CONTAINER database
sudo docker-compose stop

echo 'Clearing database..'
sudo docker exec -it $DB_CONTAINER psql -U $PG_USER -d $PG_DATABASE -c 'TRUNCATE workspaces CASCADE' 2>&1 >/dev/null


pushd seed_db
# User docker
cargo run -- ../user/toml_provider/res/seed_db.toml
popd

sudo docker container rm $DB_CONTAINER -f

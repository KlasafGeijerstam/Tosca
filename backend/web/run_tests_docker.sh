#!/bin/bash
missing_dep=0
dependencies=("docker" "cargo" "diesel" "openssl")
for dep in ${dependencies[*]}; do
	hash $dep &> /dev/null || { printf "$dep is missing.\n"; missing_dep=1; }
done

if (( $missing_dep == 1 )) ; then
	printf "Missing dependencies. Skipping tests.\n"
	exit 1
fi

# Start docker db
echo "[Database] Starting.."
sudo docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name tosca-test-db -d postgres

export PGUSER=postgres
export PGPASSWORD=1234
export PGHOST=localhost
export PGPORT=5432
export PGDATABASE=postgres

while ! pg_isready -d $PGDATABASE -h $PGHOST -p $PGPORT -U $PGUSER -t 10; do
    sleep 1
done

echo "[Database] Started!"

_test/all.sh

# Close docker db
echo "[Database] Shutting down.."
sudo docker stop tosca-test-db
echo "[Database] DONE!"

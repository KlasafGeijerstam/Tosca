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
echo "Starting database.."
sudo docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name tosca-test-db -d postgres

export PGUSER=postgres
export PGPASSWORD=1234
export PGHOST=localhost
export PGPORT=5432
export PGDATABASE=postgres

# Replace with loop checking that posgres is up..
sleep 2

_test/all.sh

# Close docker db
echo "Shutting down database"
sudo docker stop tosca-test-db

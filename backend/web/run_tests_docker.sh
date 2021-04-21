#!/bin/bash
missing_dep=0
dependencies=("docker" "cargo" "diesel" "openssl" "pytest")
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
export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE

while ! pg_isready -d $PGDATABASE -h $PGHOST -p $PGPORT -U $PGUSER -t 10; do
    sleep 1
done

echo "[Database] Started!"

# Run migrations
printf "[Migrations] Starting..\n"
pushd web_backend/db_connector
mv src/schema.rs src/old_schema.rs # Save old version of schema.rs

diesel --config-file diesel.toml migration --migration-dir migrations/ run

# Check if we need to use the new schema
if diff src/schema.rs src/old_schema.rs; then
	printf "[Migrations] No changes in schema.rs, using old version\n"
	mv src/old_schema.rs src/schema.rs
else
	printf "[Migrations] Changes in schema.rs, using new version\n"
	rm src/old_schema.rs
fi
popd
printf "[Migrations] DONE!\n"

# Start user provider
pushd ../user/toml_provider
printf "[User] Building\n"
cargo build
if (( $? != 0 )); then
	printf "Cargo build failure!\n"
	exit 1
fi

printf "[User] Starting\n"
cargo run res/user_db.toml 8000&
user_pid=$!
popd

# Start login provider
pushd ../login/dev_login
printf "[Login] Building\n"
cargo build
if (( $? != 0 )); then
	printf "Cargo build failure!\n"
	exit 1
fi

printf "[Login] Starting\n"
cargo run 9000 unused-client-endpoint &
login_pid=$!
popd

# Start backend
pushd web_backend
printf "[Backend] Building\n"
cargo build
if (( $? != 0 )); then
	printf "Cargo build failure!\n"
	exit 1
fi

printf "[Backend] Starting\n"
cargo run config.toml --database $DATABASE_URL &
backend_pid=$!
popd
export PYTHONWARNINGS="ignore:Unverified HTTPS request"
_test/wait_for_backend.py
printf "[Backend] STARTED!\n"

# Run tests
export PYTHONDONTWRITEBYTECODE=1
pytest

# Close the backend
kill $backend_pid
kill $user_pid
kill $login_pid
wait $backend_pid
printf "[Teardown] Backend closed\n"
wait $user_pid
printf "[Teardown] User provider closed\n"
wait $login_pid
printf "[Teardown] Login provider closed\n"

# Close docker db
sudo docker stop tosca-test-db
echo "[Teardown] Database closed"

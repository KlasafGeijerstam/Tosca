#!/bin/bash

export PYTHONWARNINGS="ignore:Unverified HTTPS request"
export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE

# Run migrations
printf "[Migrations] Starting..\n"
pushd db_connector
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
_test/wait_for_backend.py
printf "[Backend] STARTED!\n"


# Run tests
printf "[Tests] Running user tests..\n"
for test in _test/tests/*; do
	name=$(basename $test)

	output=$($test 2>&1)
	ret=$?

	printf "\t[$name]: "
	if (( $ret == 0 )); then
		printf "\e[32mPASSED\e[0m"
	else
		printf "\e[31mFAILED\e[0m"
	fi
	if [[ ! $output == "" ]]; then
		printf " \e[33mOUTPUT\e[0m\n"
		printf "%s\n" "$output" | awk '{ print "\t\t" $0 }'
	else
		printf "\n"
	fi

	docker exec -it tosca-test-db psql -U $PGUSER -d $PGDATABASE -c 'TRUNCATE workspaces CASCADE' 2>&1 >/dev/null
	if [[ $? != 0 ]]; then
		printf "WARNING! Could not clear database between tests\n"
	fi	
done
printf "[Tests] DONE!\n"


# Close the backend
printf "[Teardown] Closing backend..\n"
kill -s SIGINT $backend_pid
wait $backend_pid
printf "[Teardown] Closing user provider..\n"
kill -s SIGINT $user_pid
wait $user_pid
printf "[Teardown] Closing login provider..\n"
kill -s SIGINT $login_pid
wait $login_pid
printf "[Teardown] DONE!\n"

exit 0

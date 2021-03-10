#!/bin/bash

export PYTHONWARNINGS="ignore:Unverified HTTPS request"
export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE

# Run migrations
printf "[Migrations] Starting..\n"
pushd db_connector
diesel --config-file diesel.toml migration --migration-dir migrations/ run
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
cargo run 8000 res/user_db.toml&
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
		printf "\e[32mPASSED\e[0m\n"
	else
		printf "\e[31mFAILED\e[0m\n"
	fi

	if [[ ! $output == "" ]]; then
		printf "%s\n" $output | awk '{ print "\t\t[output]", $0 }'
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

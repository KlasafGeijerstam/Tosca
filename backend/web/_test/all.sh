#!/bin/bash

export PYTHONWARNINGS="ignore:Unverified HTTPS request"
export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST/$PGDATABASE

# Run migrations
printf "[Migrations] Starting..\n"
diesel --config-file db_connector/diesel.toml migration --migration-dir db_connector/migrations/ run
printf "[Migrations] DONE!\n"

# Start backend
printf "[Backend] Starting..\n"
pushd web_backend
cargo run config.toml &
backend_pid=$!
popd
_test/wait_for_backend.py
printf "[Backend] STARTED!\n"


# Run tests
printf "[Tests] Running user tests..\n"
for test in _test/tests/*; do
	name=$(basename $test)

	output=$($test)
	ret=$?

	printf "\t[$name]: "
	if (( $ret == 0 )); then
		printf "\e[32mPASSED\e[0m\n"
	else
		printf "\e[31mFAILED\e[0m\n"
		printf "\t$output\n"
	fi
done
printf "[Tests] DONE!.\n"


# Close the backend
printf "[Teardown] Closing backend..\n"
kill -s SIGINT $backend_pid
wait $backend_pid
printf "[Teardown] DONE!\n"

exit 0

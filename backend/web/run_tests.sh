#!/bin/bash
missing_dep=0
dependencies=("pg_virtualenv" "cargo" "diesel" "openssl" "pkg-config")
for dep in ${dependencies[*]}; do
	hash $dep &> /dev/null || { printf "$dep is missing.\n"; missing_dep=1; }
done

if (( $missing_dep == 1 )) ; then
	printf "Missing dependencies. Skipping tests.\n"
	exit 1
fi

pg_virtualenv _test/all.sh

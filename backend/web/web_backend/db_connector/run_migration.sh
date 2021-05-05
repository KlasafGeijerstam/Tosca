#!/bin/sh
until pg_isready -h $DB_HOST; do
    echo 'Awaiting database..'
    sleep 2
done

echo 'Running migrations..'
diesel migration run

FROM christophwurst/diesel-cli
WORKDIR /app

RUN apt update
RUN apt install --yes postgresql-client-11

COPY run_migration.sh .

ENTRYPOINT ["./run_migration.sh"]

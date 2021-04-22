FROM christophwurst/diesel-cli
WORKDIR /app
COPY ./migrations/ ./migrations
COPY ./diesel.toml .

ENTRYPOINT ["diesel"]
CMD ["migration", "run"]

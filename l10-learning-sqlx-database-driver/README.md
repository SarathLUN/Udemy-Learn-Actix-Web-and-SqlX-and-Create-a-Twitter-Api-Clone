# Lesson 10: Learning SQLX database driver

1. Introduction to SQLX:
    - SQLX is a database driver, not an ORM (Object-Relational Mapping) tool.
    - with SQLX, you write raw SQL queries for database operations (select, insert, update, delete).
2. Setting up the environment:
   - use docker to set up a Postgres database.
   - ensure you have the SQL CLI and Postgres CLI installed.
3. Database migrations:
   - SQLX supports migrations for setting up and updating the database schema.
   - migrations are handled by creating migration files that track schema changes over time.
4. Creating and Modifying the Schema:
   - initial migration sets up the schema.
   - further migrations can add tables or modify the schema.
5. Executing SQL queries:
   - SQLX allows for querying with raw SQL.
   - use parameterized queries to prevent SQL injection.
   - map the query results to Rust types using SQLX's from_row trait.
6. Handling Database Transactions:
   - transactions ensure that a series of database operations either completely succeed or fail, maintaining database integrity.

## Practical Labs:

- make sure you have docker desktop running
- make sure we have `sqlx-cli` installed
```shell
brew install sqlx-cli
```
- using [docker-compose.yml](./docker-compose.yml) file to setup postgres database
- start postgres container with below command-line:
```shell
docker compose up -d --build
```
- entry the shell mode of running postgres container
```shell
docker exec -it a853 /bin/sh
```
- then we can login into postgres with the below command
```shell
psql -h localhost -p 5432 -d tester -U tester
\l # list all databases
```
- we can either use above shell to create database or use `sqlx-cli` to manage database from remote host, here I use `sqlx-cli` with below command
- make sure we are exited from above shell container and inside our normal terminal execute below command:
```shell
sqlx database create --database-url postgres://tester:tester@localhost:5432/tester
```
- then we generate migration file with below command:
```shell
sqlx migrate add profile_table
```
- this will create migration file: [20240528093335_profile_table.sql](./migrations/20240528093335_profile_table.sql)
- in this section, we create a simple profile table and insert a few records
- then we need to run the migration with below command:
```shell
sqlx migrate run --database-url postgres://tester:tester@localhost:5432/tester
```
- then we can check in out database, should have table profile with 4 records in there.
- then we want to update database to add new table without impact to existing data.
- Ok, we just go the same flow again: migration add => put sql code in new file => migration run.
```shell
sqlx migrate add message_table
sqlx migrate run --database-url postgres://tester:tester@localhost:5432/tester
```
- then we can start coding.
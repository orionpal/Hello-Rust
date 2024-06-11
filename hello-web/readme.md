# Hello Web

## Covers
- Setting up basic web server
  - get endpoint
  - post endpoint
- hitting external endpoints (written in tests)
  - get
  - post
- hitting a database
  - PostgreSQL

## Running/Testing
1. Get the backend server running ```cargo run```
2. Start up database, we're using postgreSQL
- on windows:
  - https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
  - psql application to start up server
  - note: I couldn't get this working on locally windows, but mac works fine.
- on mac:
```
  brew update
  brew install postgresql
  brew services start postgresql

  brew services stop postgresql // When you're done
```
- Make sure to create a .env file in the root project directory with DATABASE_URL=postgres://postgres@localhost:5432/
- for potential issues see this thread: https://stackoverflow.com/questions/15301826/psql-fatal-role-postgres-does-not-exist
  - CREATE USER postgres WITH SUPERUSER PASSWORD 'password'; in psql
3. Run the tests ```cargo test```
4. If you want, open index.html to see the endpoints being hit

## Uses
- Axum
- reqwest
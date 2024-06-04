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
- on mac:
```
  brew update
  brew install postgresql
  brew services start postgresql

```
- Make sure to create a .env file with DATABASE_URL=your_url_details
3. Run the tests ```cargo test```
4. If you want, open index.html to see the endpoints being hit

## Uses
- Axum
- reqwest
Other dependencies that should be in the container:
`sudo apt update`
`sudo apt install build-essential`
`sudo apt install libssl-dev`
`cargo install diesel_cli --no-default-features --features "postgres"`

## 1. Install PostgreSQL
`sudo apt-get update`
`sudo apt-get install postgresql postgresql-contrib`

## 2. Start PostgreSQL service
`sudo service postgresql start`

## 3. Check if it exists
`sudo -u postgres psql -c "\l"`

## 4. Create a new PostgreSQL database and user
`sudo -u postgres psql`
`postgres=# CREATE DATABASE <database>;`
`postgres=# CREATE USER <username> WITH PASSWORD '<password>';`
`postgres-# GRANT ALL PRIVILEGES ON DATABASE <database> TO <user>;`
`postgres-# \q`

## 5. Log into database for direct actions
`sudo -u postgres psql`
`\l`
`\c <database_name>`

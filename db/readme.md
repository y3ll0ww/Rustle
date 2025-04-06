## 1. Install PostgreSQL
`sudo apt-get update`
`sudo apt-get install postgresql postgresql-contrib`

## 2. Check if it exists
`sudo -u postgres psql -c "\l"`

## 3. Start PostgreSQL service
`sudo service postgresql start`

## 4. Create a new PostgreSQL database and user
`sudo -u postgres psql`
`postgres=# CREATE USER <username> WITH PASSWORD '<password>'`
`postgres-# GRANT ALL PRIVILEGES ON DATABASE rustle_db TO admin`
`postgres-# \q`

## 5. Log into databas for direct actions
`sudo -u postgres psql`
`\l`
`\c <database_name>`

## 1. Install PostgreSQL
`sudo apt-get update`
`sudo apt-get install postgresql postgresql-contrib`

## 2. Start PostgreSQL service
`sudo service postgresql start`

## 3. Create a new PostgreSQL database and user
`sudo -u postgres psql`
`postgres=# CREATE USER <username> WITH PASSWORD '<password>'`
`postgres-# GRANT ALL PRIVILEGES ON DATABASE rustle_db TO admin`
`postgres-# \q`

## 4. Check if it exists
`sudo -u postgres psql -c "\l"`

## 5. Start databse
`sudo service postgresql start`
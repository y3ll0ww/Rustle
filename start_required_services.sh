#!/bin/bash

# Run PostgreSQL database
sudo service postgresql start

# Run Redis server
sudo redis-server --daemonize yes
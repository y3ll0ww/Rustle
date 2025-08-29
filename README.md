# Rustle
Rustle is a backend application designed for organizing and managing projects within collaborative workspaces.

It provides a feature-rich platform for handling **users**, **workspaces**, and **projects**, making it easy for teams to structure work, coordinate effectively, and maintain clarity across multiple initiatives.

## Table of Contents
- [Introduction](#introduction)
- [Installation](#1-installation)
- [Usage](#usage)
- [API](#api)
- [Contributing](#contributing)

## Introduction
Rustle was created to simplify the process of **managing users, projects, and workspaces** in one place.

By combining project management, workspace collaboration, and role-based user control, Rustle serves as a backbone for teams that need both **flexibility** during development and **stability** in production.

Rustle is designed to run in containerized environments, which means you don’t have to worry about local setup or system dependencies. Everything runs consistently across development machines and servers.

## 1. Installation
Rustle can be run entirely with **Docker**, which makes setup and management extremely straightforward.

There are **two supported setups** depending on your needs:

1. [Development environment](#10-development-environment) – with hot reload, *Postgres*, and *Redis*.
2. [Production environment](#2-production-environment) – production-like container for running the app.

Both environments follow the same basic principles, but the development setup offers extra tools for building and testing features.

### Prerequisites
Before running Rustle, ensure you have the following installed:
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)
> Check versions with `docker --version` and `docker compose version`.

### 1.0 Development environment
| Dockerfile         | Compose yaml               |
| ------------------ | -------------------------- |
| **Dockerfile.dev** | **docker-compose.dev.yml** |

The development environment includes everything you need to start coding immediately:
- Rust runtime with hot reloading (auto-rebuild when code changes).
- Postgres for persistent relational data.
- Redis for caching and lightweight message brokering.

This setup allows developers to work quickly without affecting production data or configuration.

### 1.1 Quickstart
You have to follow these steps.

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Clone the repository using Git and move into the project | `git clone https://github.com/<your-org>/rustle.git`<br>`cd rustle` |
| 2.   | Run the `docker compose` file | `docker compose =f docker-compose.dev.yml up --build` |
| 3.   | Open an additional terminal and move into the `rustle-dev` context | `docker exec -it rustle-dev /bin/bash` |
| 4.   | Run `Diesel` migrations | `diesel migration run` |
| 5.   | Inject an admin user by running a `cargo test` | `cargo test inject_admin_user`|

### 1.2 Removing data
If you want to reset the environment (for example, to start fresh or clean corrupted data), you can shut down all services and delete the database volume:
```bash
docker compose -f docker-compose.dev.yml down -v
```
This will remove all containers and associated volumes, effectively wiping your local Rustle database.

Afterward, simply follow the [Quickstart](#11-quickstart) steps again.

### Keep in mind
Rustle uses Diesel for database migrations. You may often need to run or revert migrations when experimenting with schema changes.

Examples:
```bash
diesel migration run --database-url postgres://user:pass@db:5432/rustle
diesel migration revert --database-url postgres://user:pass@db:5432/rustle
```

Instead of adding the `database-url` in every call, this can be managed using an environment variable. Just create a `.env` and add the following:
```bash
DATABASE_URL=postgres://user:pass@db:5432/rustle
```

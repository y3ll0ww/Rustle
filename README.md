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

---
### 1.0 Quickstart
Before running Rustle, ensure you have installed the following **prerequisites**:
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)
> Check versions with `docker --version` and `docker compose version`.

Next, follow the steps as described below:
| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Clone the repository using Git | `git clone https://github.com/y3ll0ww/Rustle.git` |
| 2.   | Get into the project folder    | `cd rustle` |
| 2.   | Run the `docker compose` file  | `docker compose -f {DOCKER_COMPOSE_YAML} up --build` |
> The *DOCKER_COMPOSE_YAML* is any compose file as described in the [development](#11-development-environment) and [production](#12-production-environment) sections.
---
### 1.1 Development environment
| Dockerfile         | Compose yaml               |
| ------------------ | -------------------------- |
| **Dockerfile.dev** | **docker-compose.dev.yml** |

The development environment includes everything you need to start coding immediately:
- Rust runtime with hot reloading (auto-rebuild when code changes).
- Postgres for persistent relational data.
- Redis for caching and lightweight message brokering.

This setup allows developers to work quickly without affecting production data or configuration.

#### Using toolchains
The development environment doesn't automatically run database migrations. The following steps described in this section are therefore critical.

To make use of the toolchains inside the development container, you'll first need to move inside the container context. After that, it will be possible to, for example, run tests.

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Move into the `rustle-dev` context | `docker exec -it rustle-dev /bin/bash` |
| 2.   | Run `Diesel` migrations | `diesel migration run` |
| 3.   | Run a test using `cargo` | `cargo test `|
> The test in the example (*inject_admin_usern be replaced with any test in the `src/tests` module.

---
### 1.2 Production environment
| Dockerfile          | Compose yaml                |
| ------------------- | --------------------------- |
| **Dockerfile.prod** | **docker-compose.prod.yml** |

The production environment is optimized for performance and portability:
- Pre-compiled Rustle binary for fast startup and minimal runtime overhead.
- Lean container image with only the dependencies required to run the application.
- Configurable via environment variables for seamless deployment in different infrastructures.

This setup ensures consistent, reproducible builds and a lightweight footprint, making it ideal for running in staging or production clusters.

The production container, database migrations are already run. After [running](#10-quickstart) the compose file, it is ready to go.

---
### 1.1 Quickstart
You have to follow these steps.

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Clone the repository using Git and move into the project | `git clone https://github.com/<your-org>/rustle.git`<br>`cd rustle` |
| 2.   | Run the `docker compose` file | `docker compose -f docker-compose.dev.yml up --build` |
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

### 2.1 Quickstart
You have to follow these steps.

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Clone the repository using Git and move into the project | `git clone https://github.com/<your-org>/rustle.git`<br>`cd rustle` |
| 2.   | Run the `docker compose` file | `docker compose -f docker-compose.dev.yml up --build` |
| 3.   | Open an additional terminal and move into the `rustle-dev` context | `docker exec -it rustle-dev /bin/bash` |
| 4.   | Run `Diesel` migrations | `diesel migration run` |
| 5.   | Inject an admin user by running a `cargo test` | `cargo test inject_admin_user`|

### 1.2 Removing data
If you want to reset the environment (for example, to start fresh or clean corrupted data), you can shut down all services:
```bash
docker compose -f docker-compose.dev.yml down
```
> The *Docker compose file* is any compose file as described in the [development](#11-development-environment) and [production](#12-production-environment) section.

> Optionally add the `-v` flag. This will remove all containers and associated volumes, effectively wiping your local Rustle database.

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

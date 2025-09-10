# Rustle

- [Introduction](#introduction)
- [1. Installation](#1-installation)
  - [1.0 Quickstart](#10-quickstart)
  - [1.1 Development Environment](#11-development-environment)
  - [1.2 Production Environment](#12-production-environment)
  - [1.3 Removing Data](#13-removing-data)
- [2. Usage](#2-usage)
- [3. API](#3-api)
- [4. Contributing](#4-contributing)

# Introduction
Rustle is a backend application designed for organizing and managing projects within collaborative workspaces.

It provides a feature-rich platform for handling **users**, **workspaces**, and **projects**, making it easy for teams to structure work, coordinate effectively, and maintain clarity across multiple initiatives.

Rustle was created to simplify the process of **managing users, projects, and workspaces** in one place.

By combining project management, workspace collaboration, and role-based user control, Rustle serves as a backbone for teams that need both **flexibility** during development and **stability** in production.

Rustle is designed to run in containerized environments, which means you don’t have to worry about local setup or system dependencies. Everything runs consistently across development machines and servers.

# 1. Installation
Rustle can be run entirely with **Docker**, making setup and management straightforward and consistent across machines.

Before running Rustle, ensure you have installed the following **prerequisites**:
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)
> Check versions with `docker --version` and `docker compose version`.

---
## 1.0 Quickstart

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Clone the repository     | `git clone https://github.com/y3ll0ww/Rustle.git` |
| 2.   | Enter the project folder | `cd rustle` |
| 3.   | Copy environment file    | `cp .env.example .env` |
| 2.   | Start the services       | `docker compose -f {DOCKER_COMPOSE_YAML} up --build` |
> The *DOCKER_COMPOSE_YAML* is any compose file as described in the [development](#11-development-environment) and [production](#12-production-environment) sections.

There are **two supported setups** depending on your needs:

1. [Development environment](#10-development-environment) – with hot reload, *Postgres*, and *Redis*.
2. [Production environment](#2-production-environment) – production-like container for running the app.

Both environments follow the same basic principles, but the development setup offers extra tools for building and testing features.

---
## 1.1 Development Environment
| Dockerfile         | Compose yaml               |
| ------------------ | -------------------------- |
| **Dockerfile.dev** | **docker-compose.dev.yml** |

The development environment includes everything you need to start coding immediately:
- **Rust runtime** with hot reloading (auto-rebuild when code changes).
- **Postgres** for persistent relational data.
- **Redis** for caching and lightweight message brokering.

This setup allows developers to work quickly without affecting production data or configuration.

### Using toolchains
The development environment doesn't automatically run database migrations. The following steps described in this section are therefore critical.

To make use of the toolchains inside the development container, you'll first need to move inside the container context. After that, it will be possible to, for example, run tests.

| Step | Description | CLI |
| ---- | ----------- | --- |
| 1.   | Open a shell in the dev container | `docker exec -it rustle-dev /bin/bash` |
| 2.   | Run Diesel migrations             | `diesel migration run` |
| 3.   | Run a test using Cargo            | `cargo test inject_admin_user` |
> The test in the example (*inject_admin_user*) can be replaced with any test in the `src/tests` module.

---
## 1.2 Production environment
| Dockerfile          | Compose yaml                |
| ------------------- | --------------------------- |
| **Dockerfile.prod** | **docker-compose.prod.yml** |

The production environment is optimized for performance and portability:
- **Pre-compiled Rustle binary** for fast startup and minimal runtime overhead.
- **Lean container image** with only the dependencies required to run the application.
- **Configurable via environment variables** for seamless deployment in different infrastructures.

This setup ensures consistent, reproducible builds and a lightweight footprint, making it ideal for running in staging or production clusters.

Unlike development, **database migrations are run automatically** during container startup.
Once the compose file is up, Rustle is ready to serve requests immediately.

---
## 1.3 Removing Data
If you want to reset the environment (for example, to start fresh or clean corrupted data), you can shut down all services and delete the database volume:
```bash
docker compose -f docker-compose.dev.yml down -v
```
This will remove all containers and associated volumes, effectively wiping your local Rustle database.

---
# 2. Usage
> TODO

---
# 3. API
> TODO

---

# 4. Contributing
> TODO

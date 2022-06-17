# ![RealWorld Example App](logo.png) <!-- omit in toc -->

[![codecov](https://codecov.io/gh/corrieriluca/realworld-actix-web/branch/main/graph/badge.svg?token=W3M8HKYJS8)](https://codecov.io/gh/corrieriluca/realworld-actix-web) ![mit](https://img.shields.io/badge/lisence-MIT-green) ![rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=black)

> ### [Actix Web](https://actix.rs/) codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.


### [Demo](https://demo.realworld.io/)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld) <!-- omit in toc -->

This codebase was created to demonstrate a fully fledged fullstack application named **Conduit** (Medium-like) built with **Actix Web** (4.0) including CRUD operations, authentication, routing, pagination, and more.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

## Other tools used <!-- omit in toc -->

- [SQLx](https://github.com/launchbadge/sqlx), 🧰 *The Rust SQL Toolkit*
- [PostgreSQL](https://www.postgresql.org/), *The World's Most Advanced Open Source Relational Database*
- [Docker](https://www.docker.com/) 🐳
- [GitHub Actions](https://github.com/features/actions) ⚙️

## Table of contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Getting started](#getting-started)
  - [🔨 With Cargo and Docker (recommended)](#-with-cargo-and-docker-recommended)
    - [🧪 Run Tests](#-run-tests)
  - [📦 With Docker Compose](#-with-docker-compose)
- [How does it work?](#how-does-it-work)
  - [Configuration files](#configuration-files)
  - [Authentication middleware](#authentication-middleware)
  - [Code architecture](#code-architecture)
- [Resources](#resources)

# Introduction

**This project is a partial implementation of the RealWorld spec.**

I aimed to use as much as good practices as I could from the Rust community. I also extensively used functional and unit testing in order to have a great code coverage.

GitHub Actions is used to test the whole project, audit security of supply chain and continuously deliver containers.

What is implemented is the following:
- [x] JWT Authentication
- [x] Users API (Registration, Login, Update)
- [x] Profiles API (Get, Follow, Unfollow)
- [ ] Articles API (List, Feed, Comments, Tags...)

# Getting started

## 🔨 With Cargo and Docker (recommended)

Your first need to install the SQLx CLI:
```
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

You need PostgreSQL running somewhere, and to run migrations on it. To run on localhost, Docker is the preferred way:
```
docker run \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=conduit \
  -p 5432:5432 \
  -d postgres:14 \
  postgres -N 1000

sqlx database create
sqlx migrate run
```

**💡 Tip:** For faster deployment you can use the [*dev_env.sh*](./scripts/dev_env.sh) script.

Finally you can run the API on port 8080:
```
cargo run
```
And in another terminal (or in [Postman](https://www.postman.com/)):
```
curl -i -X POST http://127.0.0.1:8080/api/users \
    -H "Content-Type: application/json" \
    --data '{"user":{"username":"john","email":"john.doe@github.com","password":"test1234"}}'
```

### 🧪 Run Tests

Make sure a database a Postgres database is running on localhost.

Just run:
```
cargo test
```

## 📦 With Docker Compose

Running with Docker Compose is fairly simple but not very flexible during development (requires to build the API image for each change of the source code).

Just go into the `docker` folder, and run the stack:
```
cd docker/
docker compose up -d
```

The API will be exposed on port 8080, you can run your own tests with tools such as `curl` or Postman.

# How does it work?

## Configuration files

## Authentication middleware

## Code architecture

# Resources

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
  - [⚙️ Configuration files](#️-configuration-files)
  - [🔑 Authentication middleware](#-authentication-middleware)
  - [🏛 Code architecture](#-code-architecture)
  - [🧪 Functional Tests (API)](#-functional-tests-api)
- [Resources & Bibliography](#resources--bibliography)

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

## ⚙️ Configuration files

The [`config`](https://crates.io/crates/config) crate is used to provide a convenient **layered configuration system** (satisfies [the third of the 12-factor](https://12factor.net/config)).

The config files are written in YAML and are placed under the [`configuration`](./configuration/) folder in the same directory as the app.

You can see an example of layers with the `base.yml` file which contains a default listening port and credentials for the database, and the `local.yml` and `production.yml` that overrides some settings from the base as well as adding others (such as the listening address).

Config values can also be overriden with **environment variables** (highest precedence) following this naming convention: `CONDUIT__<settings-category>__<setting>`.

**Example:** override the DB hostname with `CONDUIT__DATABASE__HOST`.

## 🔑 Authentication middleware

Almost all API endpoints require authentication (see [RealWorld backend specs](https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints)) with a JWT token.

This implementation thus makes use of the [`jsonwebtoken`](https://crates.io/crates/jsonwebtoken) crate coupled with a custom authentication middleware that wraps endpoints which need authentication and validate (or reject) requests before they arrive to the endpoint (adding authenticated user information on top of the request).

Please take a look at the [`auth.rs`](./src/middlewares/auth.rs) source file if you want to know more about the implementation of the middleware, code is documented.

## 🏛 Code architecture

The source code of this implementation resides in the [`src`](./src/) directory:

```
src
├── domain
│   └── ...
├── dtos
│   └── ...
├── handlers
│   └── ...
├── middlewares
│   ├── auth.rs
│   └── mod.rs
├── repositories
│   └── ...
├── configuration.rs
├── lib.rs
├── main.rs
└── startup.rs
```

The `conduit` library is exposed through the `lib.rs` file.

The `main.rs` source file use the `configuration` and `startup` modules to respectively configure and run the application on the desired listening address and port.

The `middlewares` module contains middlewares such as the Authentication middleware described above.

The `repositories` module contains exposed functions doing SQL queries to the database.

The `dtos` module contains Data Transfer Objects (DTOs) for defining input and output types (as `struct`) of the API.

The `domain` module contains functions and modules dealing with business logic (input validation, JWT tokens...) for the API.

Finally, the `handlers` module contains functions and modules for the **handlers**: the functions mapped to API endpoints. These handlers use the `dtos`, `domain` and `repositories` modules for their internal logic.

## 🧪 Functional Tests (API)

Under the [`tests`](./tests/) folder, you will find all the functional tests of the application, this means API tests.

Especially, the [`reqwest`](https://crates.io/crates/reqwest), [`claim`](https://crates.io/crates/claim) and [`tokio`](https://tokio.rs/) crates are used for HTTP requests, assert functions and background tasks (e.g.: running a test server) respectively.

Before each test, a database with a random name is created, SQLx migrations are runned against it and an API server is launched in background on a random port (this is called a `TestApp` within the code). The details are in the `helpers.rs` source file.

# Resources & Bibliography

- [Zero To Production In Rust](https://www.zero2prod.com/) and [A learning journal](https://www.lpalmieri.com/) from [Luca Palmieri](https://github.com/LukeMathWalker), really helped me to catch good practices on Rust Web dev tooling (such as Actix, SQLx...).
- [Demystifying Actix Web Middleware - Daniel Imfeld](https://imfeld.dev/writing/actix-web-middleware), helped me to write the Authentication middleware with the last version of Actix (v4).
- [Actix website](https://actix.rs/) and its crate [documentation](https://docs.rs/actix-web/latest/actix_web/).

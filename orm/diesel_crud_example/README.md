# Sample CRUD API with Diesel and Actix Web
This project is a simple CRUD (Create, Read, Update, Delete) API implemented in Rust using the Actix Web framework and the Diesel ORM for interacting with a PostgreSQL database. It provides basic functionalities to manage samples data, including creating, retrieving, updating, and deleting sample records.

## Prerequisites
Before getting started, make sure you have the following installed on your system:

- Rust (https://www.rust-lang.org/tools/install)
- Docker (https://www.docker.com/get-started)

## Setup

1. Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features "postgres"
```
2. Set up the database:

- Start the PostgreSQL database using Docker:
```bash
docker-compose up -d
```

- Create the database schema:
```bash
diesel setup
```

3. Set environment variables:

- Create a .env file in the root directory and add the following content with your desired database credentials:

```bash
DATABASE_URL=postgres://your-db-user:your-db-password@localhost/sampledb
```bash

## Building and Running the API
- Build and run the API using Cargo:

```bash
cargo run
```

## API endpoints
- Refer the postman collection json in the root directory for all the details

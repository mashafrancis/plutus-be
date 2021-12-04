# plutus-be
Personal financial expense tracker api built with rust

## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)

## How to run

### Manual

- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or [pgAdmin](https://www.pgadmin.org/) tool
- Rename `.env.sample` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `cargo build --release`
- Run release binary in command line/terminal.
  - Windows: `target/release/plutus-be.exe`
  - Linux/UNIX: `target/release/plutus-be.exe`
- Enjoy! 😄

### Docker

- Enter into project directory
- Run `docker-compose -f docker-compose.local.yml up` for local environment or `docker-compose -f docker-compose.prod.yml up` for production environment
- Enjoy! 😄

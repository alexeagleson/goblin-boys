# Rust / Bevy / Typescript Game Template

A complete template for building a 2D grid based game with a Rust backend with Bevy engine, and Typescript frontend.

## Demo

[https://icy-shape-6464.fly.dev/](https://icy-shape-6464.fly.dev/)

## Structure

### Backend

- Serialization (serde)
- Shared types (typeshare)
- Data persistence (sqlx)
- Web server (warp)

### Frontend

- Tooling & bundling (Vite)
- UI (React)
- Game rendering (Pixi.js)

## Usage

### Local development:

A couple of Rust CLI tools are required to build the project:

For sharing Rust types with client:

```
cargo install typeshare-cli
```

For initializing the SQLite database:

```
cargo install sqlx-cli
```

Before running for the first time you must do the following to build the initial types file and initialize the database:

1. Create your env file:
```bash
# assuming linux-like os
cp .env.example .env
```
2. Update the env file by replacing DATABASE_URL with a database url, for example `sqlite://mydb.db`

3. Create and migrate the database
```bash
sqlx database create

sqlx migrate run
```

4. Generate your types
```bash
typeshare ./ --lang=typescript --output-file=client/src/utility/types.ts
```

When this is done, you can run the server with:

```
cargo run
```

Then run the client dev server (including hot reloading) with:

```
cd client
npm install
npm run dev
```

### Release Build

The easiest way to build for release is with the included Dockerfile.

You will need Docker installed for this.

```
docker build -t gamejam .

docker run -p 8080:8080 gamejam
```

Then access the game at:

[http://localhost:8080]()

Or deploy wherever you want.

### Deployment

This project can be deployed via Docker on [https://fly.io/](https://fly.io/)

Install the fly CLI and then:

```
fly auth login

fly launch (if first time, see below for fly deploy if not)
```

Fly will launch the app and create a `fly.toml` in the root directory with project config options.

Subsequent re-launches using that configuration to overwrite can be made with:

```
fly deploy
```

# License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

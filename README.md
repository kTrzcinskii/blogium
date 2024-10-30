# Blogium

Blogium is web app created with [axum](https://github.com/tokio-rs/axum) on the backend and [Vite+React](https://vite.dev/) on the frontend. For the database, [SQLite](https://www.sqlite.org/) is used.

Blogium is a site where users can read and create new blog posts. It supports markdown syntax in post content, allowing users to better express themselves. Each post can have an image attached (only `.png` file format is supported), and each user can provide their avatar (again, only `.png` is supported). All images sent to the server are saved in the file system.

## Running in docker

To run Blogium in Docker from the project's root run:

```shell
docker compose up
```

It will create two containers - one for backend and one for frontend.
By default, frontend is hosted at [http://localhost:5173](http://localhost:5173) and backend is hosted at [http://localhost:3000](http://localhost:3000).
If you want to change something with docker configuration you should check following files:

- [Dockerfile.backend](./Dockerfile.backend)
- [client/Dockerfile.frontend](./client/Dockerfile.frontend)
- [docker-compose.yml](./docker-compose.yml)

## Running locally

You can also run blogium locally on your machine.

### Prerequistes

- node >= `20`
- rust >= `1.82`
- sqlx cli >= `0.5.2`

### Frontend

For running frontend create `client/.env` file and fill it according to [client/.env.example](./client/.env.example). Then just enter `./client` directory and run

```shell
npm run dev
```

### Backend

Backend is a little bit more compliacted than frontend. You need to complete all this steps to run it:

- Create your local `SQLite` instance.
- Create `.env` file and fill it according to [.env.example](.env.example) (you need to at least use `REQUIRED` fields)
- Run sql migrations using sqlx cli: `sqlx migrate run`
- Run app with `cargo run`

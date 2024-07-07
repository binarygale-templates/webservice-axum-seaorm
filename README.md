# webservice-axum-seaorm

This is a template project for a webservice using `axum` and SeaORM to connect to a PostgreSQL database. It is based on the [`webservice-axum` template](https://github.com/binarygale-templates/webservice-axum), and very similar to the [`webservice-axum-sqlx` template](https://github.com/binarygale-templates/webservice-axum-sqlx), except that this is using SeaORM instead of a "plain" `sqlx`.

## Features

This does pretty much what [`webservice-axum-sqlx` template](https://github.com/binarygale-templates/webservice-axum-sqlx) is doing, except that it's using SeaORM.

Just like the SQLx template, this also supports automatically updating an `updated_at` column. [SeaORM does not support that yet](https://github.com/SeaQL/sea-orm/pull/854), so this is done via a database trigger that gets added by a helper function, similar to how Diesel does it. Check out the `example` table migration for an example.

There's a helper script, `./helpers/update-db-entitites.sh` to generate the entities in `./src/entities`.

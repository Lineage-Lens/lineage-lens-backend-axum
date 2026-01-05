# lineage-lens-backend-axum

## SQLx CLI

There are a few commands required to develop this application.

To apply existing migrations:
```sh
sqlx migrate run
```

To revert a migration:
```sh
sqlx migrate revert
```

To create a new migration:
```sh
sqlx migrate add -r <name>
```
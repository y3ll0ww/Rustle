Start app with everything needed (including PostgresQL and Redis):
```bash
docker compose -f docker-compose.dev.yml up --build
```
Remove Dev with volumes (database files):
```bash
docker compose -f docker-compose.dev.yml down -v
```

Get into correct context:
```bash
docker exec -it rustle-dev /bin/bash
```

Run/revert diesel migrations; change .env:
```bash
DATABASE_URL=postgres://user:pass@db:5432/rustle
```
Or:
```bash
diesel migration run --database-url postgres://user:pass@db:5432/rustle
diesel migration revert --database-url postgres://user:pass@db:5432/rustle
```

First thing inject a user through test (after migrations):
```bash
cargo test inject_admin_user
```

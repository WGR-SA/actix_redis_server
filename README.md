# actix_redis_server

## commands

### ACTIX
```bash
cargo run
```

### Docker
```bash
docker build --platform=linux/amd64 -t actix_redis_server .
docker run -p 8080:8080 --env-file .env actix_redis_server
```
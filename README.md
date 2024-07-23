# actix_redis_server

## commands

### ACTIX
```bash
cargo run
```

### Docker
```bash
docker build --platform=linux/amd64 -t ghcr.io/wgr-sa/actix_redis_server:latest .
docker push ghcr.io/wgr-sa/actix_redis_server:latest

docker run -p 8080:80 --env-file .env ghcr.io/wgr-sa/actix_redis_server:latest
```
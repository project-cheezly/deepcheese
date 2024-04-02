docker run --name cheeseboard-cron \
    --env-file ../.env \
    -v kis_token:/tmp/kis \
    --add-host host.docker.internal:host-gateway \
    cheeseboard-cron:latest

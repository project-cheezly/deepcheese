docker run --name cheese-server \
  --env-file ../../.env \
  -v kis_token:/tmp/kis_rust \
  --network cheese-network \
  cheese-server
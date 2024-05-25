FROM node:lts-alpine3.20 AS CheezlyPage

WORKDIR /app

COPY . .

RUN npm ci

RUN npm run build

RUN rm -rf src/ static/ docker-compose.yml

USER node:node

CMD ["node", "build"]

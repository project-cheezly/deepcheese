FROM node:lts-alpine3.19 AS Cheeseboard

WORKDIR /app

COPY ./front .
COPY .env .

RUN npm ci

RUN npm run build

RUN rm -rf src/ static/ docker-compose.yml

USER node:node

CMD ["node", "build"]

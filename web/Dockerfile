FROM node:13.12.0-alpine3.11 AS build

COPY . .
RUN yarn
RUN yarn build

FROM nginx:1.17.9-alpine

COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf
COPY --from=build ./dist /usr/share/nginx/html
# syntax=docker/dockerfile:1

ARG NODE_VERSION=24

FROM node:${NODE_VERSION} as base
RUN npm install --global npm@latest

WORKDIR /usr/src/app

FROM base as deps

RUN --mount=type=cache,target=/root/.npm \
    --mount=type=bind,source=scamplers-frontend/package.json,target=scamplers-frontend/package.json \
    --mount=type=bind,source=scamplers-frontend/package-lock.json,target=scamplers-frontend/package-lock.json \
    cd scamplers-frontend && \
    npm ci --omit=dev

FROM deps as build


RUN --mount=type=bind,source=scamplers-frontend/package.json,target=scamplers-frontend/package.json \
    --mount=type=bind,source=scamplers-frontend/package-lock.json,target=scamplers-frontend/package-lock.json \
    --mount=type=cache,target=/root/.npm \
    cd scamplers-frontend && \
    npm ci

COPY scamplers-frontend scamplers-frontend
COPY scamplers scamplers

ARG API_ADDRESS_SERVER
ARG API_ADDRESS_CLIENT
ARG API_PATH
ENV API_ADDRESS_SERVER="${API_ADDRESS_SERVER}" PUBLIC_API_ADDRESS_CLIENT="${API_ADDRESS_CLIENT}" PUBLIC_API_PATH="${API_PATH}"

RUN cd scamplers-frontend && npm run build

FROM base as final

ARG NODE_ENV
ENV NODE_ENV="${NODE_ENV}"

USER node

COPY scamplers-frontend/package.json .

COPY --from=deps /usr/src/app/scamplers-frontend/node_modules ./node_modules
COPY --from=build /usr/src/app/scamplers-frontend/build ./build

EXPOSE ${PORT}

CMD node build

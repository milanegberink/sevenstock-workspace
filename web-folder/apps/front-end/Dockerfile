FROM node:current-slim AS base

WORKDIR /app

RUN npm install -g pnpm

FROM base AS dev

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./

COPY apps/front-end/package.json ./apps/front-end/

RUN pnpm install --frozen-lockfile

COPY . .

ARG PUBLIC_BACKEND_URL
ENV PUBLIC_BACKEND_URL=$PUBLIC_BACKEND_URL

WORKDIR /app/apps/front-end

CMD ["pnpm", "run", "dev"]

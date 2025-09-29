FROM node:current-slim AS base

WORKDIR /app

RUN npm install -g pnpm

FROM base AS dev

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./

COPY apps/front-end/package.json ./apps/front-end/

RUN pnpm store prune && pnpm install --frozen-lockfile

COPY . .

CMD ["pnpm", "--filter", "front-end", "dev"]

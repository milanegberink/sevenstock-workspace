FROM node:current-slim AS base

WORKDIR /app

RUN npm install -g pnpm

FROM base AS builder

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./

COPY apps/front-end/package.json ./apps/front-end/

RUN pnpm install --frozen-lockfile

COPY . .

RUN pnpm --filter front-end build

RUN pnpm --filter front-end deploy --prod /prod/app

FROM node:current-slim AS runner

WORKDIR /app

COPY --from=builder /prod/app .

EXPOSE 3000

CMD ["node", "build/index.js"]

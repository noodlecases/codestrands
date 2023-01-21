##### Build Image #####
FROM node:16 AS builder

WORKDIR /codestrands

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build

##### Runtime Image #####
FROM node:16

ENV NEXT_TELEMETRY_DISABLED 1
ENV NODE_ENV production

WORKDIR /usr/src/app

COPY --from=builder /codestrands/public ./public
COPY --from=builder /codestrands/dist ./dist
COPY --from=builder /codestrands/next.config.js ./next.config.js
COPY --from=builder /codestrands/.next/ ./.next
COPY --from=builder /codestrands/node_modules/ ./node_modules
COPY --from=builder /codestrands/package.json/ ./package.json

CMD ["sh", "-c",  "yarn start"]

# Comprehensive Dockerfile Syntax Test
# Syntax directive
# syntax=docker/dockerfile:1.4

# Base Image
FROM ubuntu:22.04 AS base
LABEL maintainer="oak-tester <test@example.com>"
LABEL version="1.0"
LABEL description="Comprehensive Dockerfile Test Image"

# Arguments and Environment Variables
ARG DEBIAN_FRONTEND=noninteractive
ARG APP_VERSION=1.0.0
ENV NODE_ENV=production \
    PORT=8080 \
    PATH="/app/node_modules/.bin:$PATH"

# Shell selection
SHELL ["/bin/bash", "-c"]

# Installation (Run commands)
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Workdir
WORKDIR /app

# Copy files
COPY package*.json ./
COPY .npmrc ./

# Conditional execution (Mounts)
RUN --mount=type=cache,target=/root/.npm \
    npm install --ci --only=production

# Multi-stage build: Builder
FROM base AS builder
ENV NODE_ENV=development
RUN npm install
COPY . .
RUN npm run build

# Multi-stage build: Final
FROM base AS final
WORKDIR /app

# Copy from builder stage
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/public ./public

# User configuration
RUN groupadd -r appuser && useradd -r -g appuser -d /app appuser
RUN chown -R appuser:appuser /app
USER appuser

# Volumes
VOLUME ["/app/data", "/app/logs"]

# Networking
EXPOSE 8080 9090

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Entrypoint and Cmd
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["node", "dist/server.js"]

# Comments with special characters
# This is a comment with UTF-8: 你好 world
# This is a comment with URL: https://example.com

# OnBuild instructions
ONBUILD COPY . /app/src
ONBUILD RUN npm install

# Stop Signal
STOPSIGNAL SIGTERM

# HereDocs (Dockerfile 1.4+)
RUN <<EOF
echo "Hello World" > /app/hello.txt
cat /app/hello.txt
EOF

RUN <<PYTHON
import os
print(f"Current dir: {os.getcwd()}")
PYTHON

# Add instruction (remote URL)
ADD https://example.com/config.json /app/config/

# Meta arguments
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN echo "Building on $BUILDPLATFORM, targeting $TARGETPLATFORM"

# Docker file for testing lexer (alternative format)
# This file contains various Docker kind elements for oak-dockerfile project

FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV APP_HOME=/app
ENV NODE_VERSION=18.17.0

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    python3 \
    python3-pip \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR $APP_HOME

# Copy package files
COPY package*.json ./
COPY requirements.txt ./

# Install Python dependencies
RUN pip3 install -r requirements.txt

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@latest

# Install Node.js dependencies
RUN npm ci --only=production

# Copy application code
COPY . .

# Build application
RUN npm run build

# Create non-root user
RUN useradd -r -u 1001 -g users -d $APP_HOME -s /sbin/nologin appuser && \
    chown -R appuser:users $APP_HOME

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Set environment for production
ENV NODE_ENV=production
ENV PYTHON_ENV=production

# Start application
CMD ["python3", "app.py"]

# Alternative CMD formats
# CMD ["npm", "start"]
# CMD ["node", "server.js"]
# CMD ["gunicorn", "--bind", "0.0.0.0:8080", "app:app"]
FROM oven/bun:1-alpine

WORKDIR /app

# Copy package files
COPY package.json ./
COPY tsconfig.json ./

# Install dependencies
RUN bun install

# Copy source code
COPY . .

# Run with Bun's native TypeScript support and watch mode
CMD ["bun", "--watch", "src/index.ts"]

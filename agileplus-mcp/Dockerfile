FROM python:3.12-slim@sha256:46cb7cc2877e60fbd5e21a9ae6115c30ace7a077b9f8772da879e4590c18c2e3

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/

WORKDIR /app

# Copy dependency files first for layer caching
COPY pyproject.toml .python-version ./

# Install dependencies (no dev extras in production image)
RUN uv sync --no-dev

# Copy source code
COPY src/ ./src/

# Run the MCP server
CMD ["uv", "run", "python", "-m", "agileplus_mcp"]

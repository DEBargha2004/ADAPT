# ==============================================================================
# ADAPT ML Engine (Python) Dockerfile
# Cross-platform development image with hot reloading and shared model storage
# ==============================================================================
FROM python:3.11-slim-bookworm

# Install build dependencies for DuckDB, PyArrow, LightGBM, and scientific Python
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Upgrade pip and install development and hot-reloading file watcher
RUN pip install --no-cache-dir --upgrade pip setuptools wheel watchdog

# Copy pyproject.toml first for efficient Docker layer caching
COPY ml/pyproject.toml /app/ml/pyproject.toml

# Install dependencies with CPU PyTorch as default for universal portability across OSes
# (Linux, macOS Apple Silicon / Intel, Windows WSL2/Docker Desktop)
# When NVIDIA GPU is present and nvidia-container-toolkit is enabled, CUDA packages can be utilized
RUN pip install --no-cache-dir torch --index-url https://download.pytorch.org/whl/cpu && \
    pip install --no-cache-dir -e "/app/ml[dev]"

ENV PYTHONUNBUFFERED=1
ENV PYTHONPATH=/app

# Copy entrypoint script
COPY deployments/docker/ml-entrypoint.sh /usr/local/bin/ml-entrypoint.sh
RUN chmod +x /usr/local/bin/ml-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/ml-entrypoint.sh"]
CMD ["watch"]


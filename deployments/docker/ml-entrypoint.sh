#!/usr/bin/env bash
# ==============================================================================
# Entrypoint for ADAPT ML Container
# Supports development, automated training pipelines, and interactive Jupyter
# ==============================================================================
set -euo pipefail

# Ensure shared model and data storage directories exist
mkdir -p /app/models /app/ml/data

# Ensure ml package is installed in editable mode for live hot reload
if [ -f "/app/ml/pyproject.toml" ]; then
    pip install --no-build-isolation --no-deps -e /app/ml >/dev/null 2>&1 || true
fi

case "${1:-shell}" in
    jupyter)
        echo "Starting Jupyter Lab on port 8888..."
        exec jupyter lab \
            --ip=0.0.0.0 \
            --port=8888 \
            --no-browser \
            --allow-root \
            --ServerApp.token='' \
            --ServerApp.password='' \
            --ServerApp.allow_origin='*'
        ;;
    watch)
        echo "Starting ML watchdog watcher for file changes in /app/ml..."
        exec watchmedo shell-command \
            --patterns="*.py;*.yaml;*.sh" \
            --recursive \
            --directory=/app/ml \
            --command='echo "Changes detected in ML module, checking formatting/tests..." && pytest /app/ml || true'
        ;;
    train)
        echo "Executing full ML offline training pipeline..."
        exec bash /app/ml/scripts/train_pipeline.sh
        ;;
    shell)
        echo "Starting interactive ML development shell..."
        exec /bin/bash
        ;;
    *)
        exec "$@"
        ;;
esac


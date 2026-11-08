.PHONY: all build test bench run proto infra-up infra-down obs-up obs-down train clean \
        docker-up docker-dev docker-down docker-logs docker-build docker-check

all: build

# ── Infrastructure (Redis + Redpanda) ────────────────────────
infra-up:
	@echo "Starting core infrastructure (Redis + Redpanda + Console)..."
	docker compose up -d redis redpanda redpanda-console

infra-down:
	@echo "Stopping core infrastructure..."
	docker compose down

infra-clean:
	@echo "Tearing down infrastructure and deleting persistent volumes..."
	docker compose down -v

# ── Observability (Prometheus + Grafana) ─────────────────────
obs-up:
	@echo "Starting observability stack (Prometheus + Grafana)..."
	docker compose --profile obs up -d prometheus grafana

obs-down:
	@echo "Stopping observability stack..."
	docker compose --profile obs stop prometheus grafana

# ── Docker Full Stack / Containerized Verification ───────────
docker-up:
	@echo "Starting all services including containerized app services..."
	docker compose --profile container --profile obs up -d

docker-dev:
	@echo "Starting full stack with live output..."
	docker compose --profile container --profile obs up

docker-down:
	@echo "Stopping all services..."
	docker compose --profile container --profile obs down

docker-logs:
	@echo "Tailing container logs..."
	docker compose logs -f

docker-build:
	@echo "Building container images..."
	docker compose --profile container build

docker-check:
	@echo "Checking Rust serving compilation inside container..."
	docker compose --profile container run --rm serving cargo check --manifest-path serving/Cargo.toml

# ── Rust Serving Engine (Host Native) ───────────────────────
build:
	@echo "Building ADAPT serving engine..."
	cd serving && cargo build --release

test:
	@echo "Running serving engine tests..."
	cd serving && cargo test

bench:
	@echo "Running latency benchmarks..."
	cd serving && cargo bench

run:
	@echo "Starting ADAPT serving engine..."
	cd serving && cargo run --release -- --config ../configs/local.yaml

proto:
	@echo "Generating protobuf code..."
	@echo "TODO: protoc generation for Rust (tonic-build) and Python (grpcio-tools)"

# ── Python ML Pipeline (Host Native) ────────────────────────
train:
	@echo "Running full offline training pipeline..."
	cd ml && bash scripts/train_pipeline.sh

synth-data:
	@echo "Generating synthetic dataset..."
	cd ml && python -m ml.data.synthetic

calibrate:
	@echo "Running probability calibration..."
	@echo "TODO: calibration entry point"

ope:
	@echo "Running off-policy evaluation..."
	@echo "TODO: OPE entry point"

# ── Cleanup ─────────────────────────────────────────────────
clean:
	@echo "Cleaning build artifacts..."
	cd serving && cargo clean
	rm -rf models/*.onnx
	rm -rf ml/data/*.parquet

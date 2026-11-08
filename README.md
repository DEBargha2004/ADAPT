# ADAPT: Real-Time Ad Decisioning & Ranking Platform

> High-throughput, ultra-low-latency (< 50 ms p99) ad decisioning and ranking platform built from scratch.

---

## ⚡ Quick Start

### Prerequisites
- [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/)
- *(Optional for host builds)* Rust >= 1.80, Python >= 3.10

### Full Stack Execution (Hot Reloading Active)

```bash
# Start full platform (serving, streaming, ml, redis, redpanda, grafana)
make docker-up
# or: docker compose up -d

# Follow service logs
make docker-logs
# or: docker compose logs -f serving

# Verify Rust compilation inside container
make docker-check

# Stop stack
make docker-down
```

### Infrastructure Only
```bash
# Start Redis, Redpanda, Prometheus, Grafana
make infra-up

# Stop infrastructure
make infra-down
```

---

## 🌐 Port Matrix

| Service | Port | Description |
|---|---|---|
| **Serving Engine** | `50051` | gRPC AdDecisionService hot path |
| **Serving Metrics** | `9091` | Prometheus scrape endpoint |
| **Redis** | `6379` | Feature store & budget counters |
| **Redpanda Kafka** | `9092` | Event ingestion broker |
| **Redpanda Console**| `8080` | Topic management UI (`http://localhost:8080`) |
| **Prometheus** | `9090` | Metrics dashboard (`http://localhost:9090`) |
| **Grafana** | `3000` | Dashboards (`http://localhost:3000`, user/pass: `admin`/`admin`) |
| **ML Engine** | `8888` | Optional interactive Jupyter Lab |

---

## 🏗️ Directory Layout

```
.
├── serving/          # Rust hot-path engine (< 50 ms p99 SLA)
├── ml/               # Python offline ML, training & evaluation
├── streaming/        # Rust streaming feature store consumers (Redpanda -> Redis)
├── proto/            # Protobuf schema definitions (gRPC interfaces)
├── configs/          # Runtime configs (docker.yaml, local.yaml)
├── deployments/      # Dockerfiles, Prometheus & Grafana provisioning
├── models/           # Shared storage for trained models (.onnx, .hnsw)
├── docs/             # Technical specifications & architecture details
│   ├── architecture-details.md # Deep dive into subsystems, latency budgets & algorithms
│   ├── architecture-spec.md    # Formal platform design specification
│   └── tech-stack.md           # Technology evaluation & trade-offs
├── docker-compose.yml# Multi-container orchestration
└── Makefile          # Project command runner
```

---

## 📖 Documentation

- 🏛️ **[Architecture & Engineering Details](docs/architecture-details.md)**: System flow diagram, per-stage latency budgets (< 50 ms SLA), subsystem algorithms (HNSW, DLRM, PID pacing), and hardware sizing profiles.
- 📋 **[Full Architecture Specification](docs/architecture-spec.md)**: Formal spec covering counterfactual biases, point-in-time correctness, and evaluation criteria.
- ⚙️ **[Technology Stack & Trade-Offs](docs/tech-stack.md)**: Deep dive into language selection (Rust vs. C++ vs. Go), vector indexes, and runtimes.
- 📌 **[Master Task Tracker](TASKS.md)**: Milestone and progress tracking across all development phases.

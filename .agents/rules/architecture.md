# ADAPT Architecture & Systems Rules

1. **Strict Latency Budget (< 50 ms p99 SLA)**:
   - Request parsing & user feature fetch (Redis): `< 8 ms`
   - Two-tower candidate retrieval (HNSW in-process): `< 10 ms`
   - Deep ranking inference (ONNX Runtime in-process, FP16 where it passes parity): `< 20 ms`
   - Bidding, pacing & auction resolution: `< 5 ms`
   - Buffer / network overhead: `< 7 ms`
   - **Throughput target**: prove p99 < 50 ms at **200-500 QPS** on the local RTX 3050 laptop. Anything above that (1,000+ QPS) is a measured stress curve or an analytical extrapolation, and must be labelled as such.
   - The only network hop allowed on the hot path is the budgeted Redis feature fetch. Never introduce other out-of-process RPC hops, inference sidecars, external vector DBs, or JVM runtimes.

2. **Language Boundary**:
   - **Hot Path Serving Engine**: authored exclusively in **Rust** (no GC pauses, memory safety, in-process FFI).
   - **Offline ML & Analytics**: authored in **Python** (PyTorch, DuckDB, scikit-learn, LightGBM). This includes reference implementations and golden-vector generators used to prove Rust parity. Python never runs in the serving loop.

3. **In-Process Inference & Search**:
   - DLRM/DCN-v2 inference runs inside the Rust process via `ort` (ONNX Runtime bindings). Check the installed `ort` version and API before writing code.
   - Vector search runs inside the Rust process. The ANN library is chosen by **ADR in Phase 5**, not fixed here. Candidates to evaluate: `instant-distance`, `hnsw_rs`, `usearch`, FAISS bindings. **Requirement**: targeting predicates (geo, device, age group, negative keywords) are applied during traversal, not as a post-filter, to avoid candidate starvation. Whether a given library supports this (or can be extended to) must be verified, not assumed.

4. **Correctness Invariants**:
   - **Calibrated probabilities before bidding.** pCTR enters the auction (eCPM = pCTR x bid) only after calibration; track ECE and Brier alongside AUC and log loss.
   - **Point-in-time correctness**: offline feature materialization uses DuckDB `ASOF` joins with no future data. A model trained on leaky features is considered broken.
   - **Skew test**: replaying events must produce online (Redis) feature values matching the offline recompute within a stated tolerance.
   - **Redis atomicity**: single commands (`HSET`, `INCRBY`) are atomic individually; any multi-key or multi-step update uses `MULTI/EXEC` or a Lua script. Rolling windows use time-bucketed counters.
   - **Budget and spend counters must never be evictable.** Redis runs with `noeviction` (or counters are guaranteed non-expiring and non-evictable). A silently deleted spend counter makes the pacer overbid.
   - Stream consumers are **at-least-once**: updates must be idempotent or deduplicated.

5. **Graceful Degradation**: Every stage receives the remaining deadline and degrades rather than overruns. Circuit breakers and a cheap fallback ranker guarantee a response inside the SLA. A late optimal answer earns nothing.

6. **Model Lifecycle**:
   - No model reaches serving without an ONNX **parity test** (PyTorch vs ONNX vs Rust) with justified tolerances.
   - Keep prior checkpoints for rollback. Automatic rollback triggers on PSI > 0.25 or significant calibration drift, with hysteresis to avoid flapping.

7. **Measurement & Reproducibility**:
   - Every performance or quality number is recorded with hardware, OS, build profile (`--release`), warmup, load tool, duration, request mix, and whether it ran in Docker.
   - Latency benchmarks come from native release builds. Use open-loop load generation and watch for coordinated omission.
   - All experiments are seeded; configs and seeds are stored next to outputs. Dependencies are pinned. Report negative results honestly.

8. **Environment & Compose**:
   - There is exactly **one** Docker Compose file (`docker-compose.yml` at the repo root; update this line if it lives elsewhere). Never create a second copy.
   - Compose is for infrastructure (Redis, Redpanda, optional observability) and reproducible CPU checks. Use profiles so `docker compose up` starts infrastructure only.
   - Container processes use `configs/docker.yaml`; native processes use `configs/local.yaml`. Redpanda exposes an internal listener for containers and an external one (`localhost:9092`) for native processes.
   - Pin image tags; do not use `:latest`. Bind exposed ports to `127.0.0.1`.

9. **Decisions & Dependencies**: Any choice between libraries or designs is recorded as an ADR in `docs/adr/` with at least two options and trade-offs. The learner decides.

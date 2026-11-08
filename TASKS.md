# ADAPT: TASKS.md

> Maintained by the agent, reviewed by the learner. Read `AGENT_GUIDE.md` first.
> Tiers: **A** learner writes / **B** paired (skeleton + learner fills) / **C** agent writes, learner reviews / **W** worked example in `sandbox/`.
> Rule: a phase is done only after its **gate**: acceptance tests pass, explain-backs logged, cold rebuild done, defense drill done.
> Phase order is a recommendation. Phase 11 (OPE) is pure Python and may be swapped in at any point after Phase 3 when a break from Rust is wanted.

## Active state

- **Current phase**: 0
- **Active brick**: (none yet; run `/next`)
- **Last completed**: (none)
- **Open debts**: (none)

## Roadmap at a glance

- [ ] P0 Orientation and environment
- [ ] P1 Rust foundations via a pure-logic core (auction, PID, top-K)
- [ ] P2 Synthetic world with ground truth
- [ ] P3 ML foundations and baselines (logistic regression, GBDT, metrics, calibration)
- [ ] P4 Deep ranker (DLRM, DCN-v2, multi-task, calibration, scorecard)
- [ ] P5 Two-tower retrieval and ANN
- [ ] P6 ONNX bridge and Rust inference
- [ ] P7 Async Rust and the serving skeleton (budgets, circuit breaker, fallback)
- [ ] P8 Feature store and streaming (online/offline, point-in-time joins)
- [ ] P9 Pacing and auction in the loop
- [ ] P10 Online learning, delayed feedback, drift, rollback
- [ ] P11 Off-policy evaluation
- [ ] P12 Load testing, profiling, hardening, write-up

---

## P0: Orientation and environment

**Goal**: working toolchains and a clear mental model before writing real code.
**Learn**: `rustup`/`cargo` basics, Python env, GPU visibility, what each infra service is for.

- [ ] 0.1 (C) Install/verify Rust (pinned toolchain), Python env, `nvidia-smi`, PyTorch sees the GPU, Docker Compose for Redis and Redpanda, Makefile targets. Learner reviews each file and explains one thing in it.
- [ ] 0.2 (W) "Hello cargo": a tiny crate, `cargo test`, `cargo run`, reading a compiler error.
- [ ] 0.3 (A) **Diagnostic, not graded**: learner explains in their own words, without AI, how a RAG pipeline works and guesses how an ad server differs. Used only to calibrate pace.
- [ ] 0.4 (A) One-page "ADAPT in my own words": the request path from request to winning ad, and where each latency budget applies.

**Done when**: `cargo test` green; `make` target brings up Redis and Redpanda healthy; GPU visible from PyTorch; the one-pager exists.
**Defend**: why in-process inference instead of an inference sidecar (state the trade-off, including what you lose).

---

## P1: Rust foundations via a pure-logic core

**Goal**: learn Rust on small, synchronous, easily testable pieces, with algorithms proven in Python first.
**Learn (just in time)**: ownership/moves/borrowing, `&` vs `&mut`, structs/`impl`, enums/`match`, `Option`/`Result`/`?`, traits, `Vec`/`HashMap`/iterators, modules, tests, basic `serde`, floating-point gotchas (`NaN` ordering, `f32` vs `f64`).

- [ ] 1.1 (A) Python reference: GSP auction with reserve price (eCPM = pCTR x bid; second-price rule; tie-breaks). Agent writes the test cases, learner writes the function.
- [ ] 1.2 (A) Python reference: PID pacer simulation (error = target spend vs actual spend; output alpha clamped to [0, 1]; anti-windup).
- [ ] 1.3 (C) Golden-vector exporter: Python writes JSON inputs/outputs into `golden/`.
- [ ] 1.4 (A) Rust domain types: `AdRequest`, `Candidate`, `ScoredCandidate`, `AuctionResult` (decide what is owned vs borrowed, and defend it).
- [ ] 1.5 (A) Rust `run_auction` that passes the golden vectors.
- [ ] 1.6 (A) Rust `PidPacer::update` with anti-windup, passing golden vectors.
- [ ] 1.7 (A) Rust top-K selection using a bounded binary heap (learn `BinaryHeap`, `Ord`, why `f32` is not `Ord`).
- [ ] 1.8 (C) CLI wrapper: `adapt-cli auction --requests sample.json` with `serde` I/O.
      **Done when**: golden parity tests pass; the CLI runs 10k synthetic requests; the learner explains a move vs a borrow using an example from their own code; cold rebuild of the GSP auction.
      **Defend**: PID vs uniform hourly caps; when GSP is not truthful and what VCG changes.

---

## P2: Synthetic world with ground truth

**Why**: there is no real ad dataset, and a synthetic world with **known true CTR/CVR** is what lets us prove leakage, calibration error and off-policy estimator error later.
**Learn**: sigmoid link, latent user/ad factors, position bias, logging policies and **logged propensities**, delayed conversion distributions, event-time vs processing-time, Parquet.

- [ ] 2.1 (A) Design the true CTR function: latent user and ad factors plus feature effects plus noise; target a realistic base rate and record the choice in an ADR.
- [ ] 2.2 (A) Position-bias curve and the **logging policy** (e.g. softmax over scores with a temperature) that records the propensity of each shown ad.
- [ ] 2.3 (A) Conversion model: pCVR conditional on click, with a delay distribution (seconds for clicks, hours/days for conversions).
- [ ] 2.4 (C) Parquet writer, schema, seeds, event logs sized per spec (about 100k users, 50k ads for the portfolio build). Oracle columns (true probabilities) go in a **separate** file so training code cannot see them by accident.
- [ ] 2.5 (C) Sanity plots: base rates, position effect, propensity histogram.
      **Done when**: dataset has timestamps, positions, propensities, labels; oracle file separate; the learner explains why logged propensities are essential for OPE and what happens without them.
      **Defend**: why synthetic ground truth is a feature of this project, and what it cannot tell you about real traffic.

---

## P3: ML foundations and baselines

**Learn**: loss, gradient, gradient descent, logistic regression, overfitting, **time-based** train/validation/test splits, AUC, log loss, Brier score, ECE, reliability diagrams, Platt scaling, isotonic regression, GBDT intuition.

- [ ] 3.1 (W) Gradient descent on a toy problem.
- [ ] 3.2 (A) Logistic regression from scratch in numpy: forward, loss, gradient, loop. Compare with scikit-learn within a tolerance.
- [ ] 3.3 (A) Metric functions by hand: log loss, Brier, **ECE** (binning). AUC may use a library, but the learner must explain what it measures.
- [ ] 3.4 (B) LightGBM baseline with time-based splits.
- [ ] 3.5 (A) Platt and isotonic calibration fitted on a held-out set; reliability diagrams before and after.
      **Done when**: custom LR matches sklearn; baseline scorecard v0 (AUC, log loss, Brier, ECE); using **their own data**, the learner demonstrates numerically that a 20% pCTR overestimate overbids by 20%; cold rebuild of ECE.
      **Defend**: why calibration matters more in auctions than in search ranking.

---

## P4: Deep ranker

**Learn**: tensors, autograd, `nn.Module`, embeddings for sparse IDs (relate to RAG embeddings), MLP, BCE-with-logits, Adam, batches/epochs, GPU use, mixed precision, VRAM accounting, DLRM structure (bottom MLP, embedding tables, pairwise dot-product interaction, top MLP), DCN-v2 cross layer (x_next = x0 \* (W x + b) + x, elementwise product), multi-task heads (pCTR, pCVR) and why pCVR trained only on clicks is selection-biased.

- [ ] 4.1 (W) A minimal `nn.Module` and training loop on a toy dataset.
- [ ] 4.2 (B) Dataset/DataLoader, config, checkpointing.
- [ ] 4.3 (A) Embedding + MLP ranker; first training loop; the **overfit-one-batch** sanity test.
- [ ] 4.4 (A) DLRM pairwise-dot interaction layer.
- [ ] 4.5 (A) DCN-v2 cross layer (and a stacked cross network).
- [ ] 4.6 (A) Evaluation loop plus calibration on a held-out set; peak VRAM measurement (budget: under 2 GB for training).
- [ ] 4.7 (C) Scorecard generation: GBDT vs MLP vs DLRM vs DCN-v2 on AUC, log loss, Brier, ECE, before and after calibration.
      **Done when**: scorecard exists; every number is explained by the learner, **including any case where the deep model does not beat GBDT**; VRAM peak recorded.
      **Note**: features here come from the generator. They are replaced by point-in-time features in P8, and the ranker is **retrained** then; this is a known, deliberate gap.
      **Defend**: why calibrate after training rather than hope the loss does it.

---

## P5: Two-tower retrieval and ANN

**Learn**: dual encoders, cosine/dot-product similarity, **in-batch negatives** softmax loss, sampling bias and logQ correction (concept level), recall@K, HNSW parameters (M, efConstruction, efSearch), exact kNN baseline, three filtering strategies (post-filter, pre-filter, in-traversal predicate).

- [ ] 5.1 (A) Two-tower model and in-batch softmax loss.
- [ ] 5.2 (A) Recall@K evaluation against brute-force exact search.
- [ ] 5.3 (B) Build an HNSW index in Python (prototype) and sweep efSearch: recall vs latency curve.
- [ ] 5.4 (A) **Starvation experiment**: apply a highly selective targeting filter after ANN search and show how many candidates survive; compare with in-traversal filtering.
- [ ] 5.5 (A) ADR: choose the Rust-side ANN library and index lifecycle (build offline in Python and load in Rust, or build in Rust). The agent presents at least 2 verified options with trade-offs, including whether predicate pushdown is supported.
      **Done when**: recall@500 vs exact is reported; starvation demonstrated with numbers; ADR logged; the learner explains the offline/online split of the two towers.
      **Defend**: two-tower retrieval vs a single heavy ranker over the full pool.

---

## P6: ONNX bridge and Rust inference

**Learn**: ONNX graph, opset, dynamic batch axis, FP16 conversion and its numerical effects, parity testing, `ort` sessions, tensors in Rust (`ndarray`), warmup, execution providers (CPU vs CUDA).

- [ ] 6.1 (B) Export the ranker to ONNX with a dynamic batch dimension.
- [ ] 6.2 (A) **Parity test**: PyTorch vs ONNX vs Rust outputs on 10k rows. The learner chooses and justifies tolerances (FP32 vs FP16).
- [ ] 6.3 (A) Design the Rust `Ranker` API (types, error handling, who owns the session).
- [ ] 6.4 (B) `ort` integration. The agent verifies the current `ort` version and API before writing anything.
- [ ] 6.5 (C) CUDA execution provider setup, with the steps explained. CPU execution provider is acceptable for the MVP; GPU is a performance goal.
- [ ] 6.6 (B) Criterion benchmark: batch of 500 candidates, warmup, p50/p99, CPU vs GPU.
      **Done when**: parity within tolerance; latency recorded in `reports/` with hardware details; the learner explains why FP16 can change results.
      **Defend**: in-process ONNX Runtime vs an inference server (what do you gain, what do you give up?).

---

## P7: Async Rust and the serving skeleton

**Learn**: `async/await`, futures, the `tokio` runtime, `spawn_blocking`, `Arc`, `Send`/`Sync`, channels, `tokio::time::timeout`, deadline propagation, backpressure, `tonic`/protobuf, `tracing` spans, circuit breaker states (closed, open, half-open).

- [ ] 7.1 (W) Async worked example, then a **deliberate mistake demo**: block the executor with a synchronous sleep, watch tail latency explode, then fix it with `spawn_blocking`.
- [ ] 7.2 (C) Proto definitions and `build.rs`; learner reviews the message design.
- [ ] 7.3 (B) **Thin slice**: gRPC request returns a winner using **stub** stages (fixed features, brute-force candidates, stub scorer).
- [ ] 7.4 (A) Deadline/budget propagation: each stage receives the remaining time; stages degrade rather than overrun.
- [ ] 7.5 (A) Circuit breaker state machine and the fallback decision (cached heuristic or GBDT ranker).
- [ ] 7.6 (B) Replace stubs one at a time with real stages: Redis fetch (statically seeded features for now), ANN, ONNX ranker, auction.
- [ ] 7.7 (B) Per-stage timing via `tracing`.
      **Done when**: requests return within budget; stage timings visible; **chaos test**: inject a slow ANN, the breaker trips, fallback engages, p99 stays under 50 ms; the learner explains what happens when a task blocks an async worker thread.
      **Defend**: static per-stage budgets with graceful fallback vs best-effort processing.

---

## P8: Feature store and streaming

**Learn**: online vs offline store, rolling windows (time-bucketed counters in Redis), atomicity (Lua/`MULTI`), event time vs processing time, Redpanda/Kafka concepts (topics, partitions, offsets, consumer groups, at-least-once, idempotent updates), append-only Parquet, DuckDB `ASOF JOIN`, leakage.

- [ ] 8.1 (A) Rolling-counter design (bucket scheme for 5m/1h/24h) with a written correctness argument.
- [ ] 8.2 (C) Producers, topic setup and event replay tooling.
- [ ] 8.3 (B) Rust streaming consumer updating Redis atomically. The agent presents verified Kafka-client options with trade-offs first (build requirements differ).
- [ ] 8.4 (A) Point-in-time training set via DuckDB `ASOF JOIN`: features at time T use only data before T.
- [ ] 8.5 (A) **Leakage experiment**: naive join vs point-in-time join; show the inflated offline AUC and what happens after "deployment" (replay).
- [ ] 8.6 (A) **Skew test**: replay events, compare online Redis values with the offline recompute at the same timestamps.
- [ ] 8.7 (B) Retrain the ranker on point-in-time features and update the P4 scorecard.
      **Done when**: skew test passes within a stated tolerance; the leakage demo is documented with numbers; scorecard v2 exists.
      **Defend**: dual-store point-in-time architecture vs ad-hoc SQL joins.

---

## P9: Pacing and auction in the loop

**Learn**: campaign budgets, spend velocity, target trajectories (diurnal curves), PID behavior (windup, saturation, oscillation, tuning), atomic spend counters (`INCRBY`).

- [ ] 9.1 (B) 24-hour simulated traffic harness with diurnal load.
- [ ] 9.2 (A) Run with no pacing, uniform caps, and PID; tune Kp/Ki/Kd and explain each term's effect using their own plots.
- [ ] 9.3 (A) Rust integration: read spend from Redis, apply the multiplier alpha in the bid path.
- [ ] 9.4 (C) Pacing report generation.
- [ ] 9.5 (Stretch, A) Contextual-bandit pacing agent (e.g. epsilon-greedy or LinUCB), only after the PID MVP.
      **Done when**: pacing report shows spend curves with and without PID; overspend tolerance set via ADR and met; cold rebuild of the PID step.

---

## P10: Online learning, delayed feedback, drift, rollback

**MVP scope**: micro-batch fine-tuning from the stream (not per-event SGD), plus full drift detection and rollback.
**Learn**: incremental vs full retraining, delayed conversions and fake-negative windows (and their limitations), **PSI** (binning, thresholds), **KS** test, rolling ECE, model checkpoint registry, atomic model swap in Rust, rollback rules with hysteresis.

- [ ] 10.1 (A) PSI and KS monitors (by hand), on both feature distributions and prediction outputs.
- [ ] 10.2 (A) Rolling calibration monitor.
- [ ] 10.3 (B) Micro-batch fine-tuning job from the stream, with checkpointing.
- [ ] 10.4 (A) Rollback rule: PSI above 0.25 or calibration drift beyond a threshold, with hysteresis to avoid flapping. Discuss false-positive rate.
- [ ] 10.5 (B) Atomic model hot-swap in the Rust server (the agent verifies the right primitive and explains the choice).
- [ ] 10.6 (A) **Drift demo**: inject a sudden CTR crash on a major creative category; show the alert firing, rollback, and recovery.
      **Done when**: the demo is recorded in `reports/` with timestamps; the learner explains delayed-feedback bias in their own words.
      **Defend**: continual fine-tuning vs periodic batch retraining, and the guardrails that make the former safe.

---

## P11: Off-policy evaluation

**Learn**: logged bandit feedback, propensities, IPS, self-normalized IPS, clipping, doubly robust estimation with a reward model, bias-variance trade-off, overlap/support requirement.

- [ ] 11.1 (A) IPS, SNIPS and DR estimators from scratch.
- [ ] 11.2 (A) Compare estimates against **ground-truth simulated A/B** (the oracle file from P2) across many seeds; report error and variance.
- [ ] 11.3 (A) Clipping study, plus a deliberate **low-overlap** case where IPS fails, shown with numbers.
- [ ] 11.4 (C) Report generation with confidence intervals.
      **Done when**: the learner explains when each estimator breaks; the report compares IPS/DR with ground truth.
      **Defend**: DR evaluation vs naive offline test-set accuracy.

---

## P12: Load testing, profiling, hardening, write-up

**Learn**: open-loop vs closed-loop load generation and **coordinated omission**, percentile measurement (HdrHistogram), warmup, release builds, flamegraphs, tail-latency causes.

- [ ] 12.1 (C) Load generator wiring (the agent verifies options, e.g. an existing gRPC load tool or a small Rust client with HDR histograms).
- [ ] 12.2 (A) Latency profile at 100 to 500 QPS (the SLA range), plus a stress curve to 2,000 QPS labelled "beyond SLA", with per-stage breakdown: p50, p90, p99, p99.9.
- [ ] 12.3 (A) Interpret flamegraphs; identify the top 2 bottlenecks; fix one; re-measure; revisit `TODO(perf)` debts.
- [ ] 12.4 (A) Analytical extrapolation note for multi-replica / datacenter GPUs, clearly labelled as extrapolation.
- [ ] 12.5 (C) README, architecture overview, ADR index.
- [ ] 12.6 (A) **Defense document**: 10 hard questions per area (retrieval, ranking, calibration, serving, features, pacing, drift, OPE) with the learner's own answers.
      **Done when**: all deliverables from the spec exist in `reports/` (GBDT/DLRM/DCN-v2 scorecard, pacing report, latency report, OPE validation, drift demo), with honest numbers.

---

## Decision log (ADR index)

| ADR        | Decision | Date | Revisit if |
| ---------- | -------- | ---- | ---------- |
| (none yet) |          |      |            |

## Benchmark log

| Date       | What | Hardware | Build | Method | Result |
| ---------- | ---- | -------- | ----- | ------ | ------ |
| (none yet) |      |          |       |        |        |

## Debts

- (none yet)

## Open questions

- (none yet)

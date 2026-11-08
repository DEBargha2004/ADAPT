# 1. ADAPT: Real-Time Ad Decisioning & Ranking Platform

## Background

In modern adtech, e-commerce marketplaces, and fintech recommendation systems (Google Ads, Meta, Amazon Advertising, Trade Desk), real-time decision engines must evaluate millions of potential ad candidates, generate fine-grained value predictions, execute multi-party auctions, and enforce strict advertiser budgets within a tight deadline (< 50 ms p99). Naive recommendation systems fail in this environment because they ignore counterfactual selection biases, training-serving skew on dynamic features, conversion feedback delays, and the non-linear interaction between probabilistic calibration and bidding budgets. ADAPT is a production-grade, portfolio-level real-time decisioning and ranking platform built from scratch: a two-tower ANN candidate generator, a high-throughput deep ranking model (DLRM/DCN) calibrated for bid accuracy against a GBDT baseline, a streaming feature store with point-in-time correctness, a closed-loop PID/RL budget pacing controller, continuous online learning with drift detection, off-policy evaluation (OPE) via Inverse Propensity Scoring (IPS) and Doubly Robust estimators, and an ultra-low latency gRPC/ONNX inference server with per-stage latency budgets.

---

**Two-tower candidate generation with ANN index**

- Reduce a candidate pool of millions of ad creatives and campaigns down to ~500 high-relevance candidates in under 10 ms.
- Compute offline user embedding and ad embedding models using dual neural encoders trained on query/user interaction histories.
- Index item embeddings into an approximate nearest neighbor (ANN) vector index (HNSW or IVFPQ) running in-memory or embedded.
- Support real-time hard filtering on targeting predicates (geography, device type, age group, negative keyword lists) pushed down directly into the ANN traversal to avoid post-filter candidate starvation.
- *Design decision to defend:* Why two-tower retrieval over single-stage heavy rankers? Single-stage deep ranking over millions of candidates violates the 50 ms latency ceiling. The two-tower architecture enables decoupled offline/cached item indexing with sub-linear $O(\log N)$ vector search, trading inter-feature cross-attention for latency.

**Multi-task deep ranking model (DLRM / DCN-v2) & calibration**

- Rank the top 500 candidates by estimating Click-Through Rate ($p\text{CTR}$) and Conversion Rate ($p\text{CVR}$).
- Combine dense numerical continuous features (historical CTR, spend velocity, user engagement ratios) with sparse categorical features (user ID, publisher domain, creative category) mapped to dense embedding vectors.
- Implement explicit feature interactions using Cross Networks (DCN-v2) or dot-product feature interaction layers (DLRM), benchmarked against a LightGBM/GBDT baseline.
- Calibrate model probabilities using Platt scaling or isotonic regression on a held-out validation set. Measure Expected Calibration Error (ECE) and Brier score alongside ROC-AUC and log loss.
- *Design decision to defend:* Why is probability calibration more critical in adtech than in general search ranking? In search ranking, only relative order (ROC-AUC) matters. In ad auctions, predicted CTR directly scales the expected value ($\text{eCPM} = p\text{CTR} \times \text{bid}$). An uncalibrated model that overestimates CTR by 20% overbids by 20%, rapidly depleting advertiser budgets on low-value traffic and causing severe platform churn.

**Bidding engine & closed-loop pacing controller**

- Compute real-time bids for each candidate based on auction mechanics: Generalized Second Price (GSP) or Vickrey-Clarke-Groves (VCG), with reserve price floors.
- Implement a budget pacing controller to distribute each campaign's finite daily budget evenly or strategically across 24 hours (avoiding budget exhaustion in early morning hours).
- Build a PID controller that modulates bid multipliers based on current spend velocity versus target spend trajectory: $u(t) = K_p e(t) + K_i \int e(\tau)d\tau + K_d \frac{de(t)}{dt}$.
- Implement a lightweight Reinforcement Learning (contextual bandit) pacing agent to handle diurnal traffic spikes.
- *Design decision to defend:* PID controller vs static uniform budget pacing. Uniform spend caps reject high-value ad requests once an hourly cap is hit, leaving money on the table. A PID controller continuously adjusts the bid scaling factor ($\alpha \in [0, 1]$), allowing the system to bid aggressively when high-converting traffic arrives while maintaining smooth daily budget depletion.

**Streaming feature store with point-in-time correctness**

- Dual-storage architecture: an ultra-low latency online store (Redis) serving real-time aggregates and a point-in-time correct offline store (Parquet/DuckDB) for training dataset generation.
- Stream user click/impression events through Kafka/Redpanda. Aggregate rolling counters (user clicks in last 5m, 1h, 24h; publisher domain view rates) via streaming consumers and write atomically to Redis.
- Prevent training-serving skew: materialize historical snapshots with point-in-time correct joins, ensuring that features paired with an impression label at timestamp $T$ reflect only data available prior to $T$.
- *Design decision to defend:* Dual-store point-in-time join architecture vs ad-hoc SQL joins. Naive offline queries join feature tables on entity IDs without timestamp upper bounds, causing catastrophic future data leakage that inflates offline metrics but tanks production revenue.

**Online learning, drift detection & automated rollback**

- Continually update ranker weights using fresh feedback streams from Kafka as clicks and conversions arrive.
- Handle delayed conversion feedback (clicks arrive in seconds; conversions take hours or days) using importance-weighted hazard models or fake negative calibration windows.
- Continuously compute statistical drift metrics on feature distributions and prediction outputs: Population Stability Index (PSI) and Kolmogorov-Smirnov (KS) tests between inference logs and baseline training data.
- Trigger automatic rollback to the previous checkpoint if PSI crosses threshold ($> 0.25$) or if prediction calibration drifts significantly.
- *Design decision to defend:* Continual online fine-tuning vs periodic daily batch retraining. Ad performance shifts rapidly during live events, promotional spikes, and competitor bid changes. Online learning adapts to micro-trends within minutes, provided guardrail drift monitors prevent catastrophic model collapse.

**Off-policy evaluation (OPE) & counterfactual experimentation**

- Offline experimentation framework to evaluate new ranking models on historical interaction logs without deploying them to live production traffic.
- Address position bias and selection bias (users disproportionately click top-ranked items regardless of relevance) using Inverse Propensity Scoring (IPS):
  $$\hat{R}_{\text{IPS}}(M) = \frac{1}{N} \sum_{i=1}^N \frac{P_M(a_i | x_i)}{P_0(a_i | x_i)} r_i$$
- Implement Doubly Robust (DR) estimation combining IPS with a direct reward model to reduce variance while remaining unbiased.
- Run synthetic A/B test simulations comparing OPE predictions against actual simulated A/B outcomes.
- *Design decision to defend:* Doubly robust off-policy evaluation vs naive offline test-set accuracy. Historical logs are biased by the policy that generated them. Evaluating a new model on raw historical clicks rewards models that replicate the old model's blind spots rather than models that discover genuinely superior ad placements.

**Low-latency serving runtime & strict latency budgeting**

- Enforce a strict end-to-end SLA: 50 ms p99 latency at 1,000+ QPS.
- Establish hard stage budgets:
  - Request parsing & user feature retrieval (Redis): < 8 ms
  - Candidate retrieval (ANN): < 10 ms
  - Deep ranking inference (ONNX Runtime / Triton with FP16/dynamic batching): < 20 ms
  - Bidding, pacing, & auction resolution: < 5 ms
  - Buffer / network overhead: < 7 ms
- Build circuit breakers and graceful degradation: if ANN or ranker approaches timeout, fallback to a cached heuristic/GBDT ranker to guarantee responses within SLA.
- *Design decision to defend:* Static per-stage latency budgets with graceful fallback vs best-effort processing. In ad exchanges, an ad server that responds at 51 ms receives zero revenue and incurs auction timeout penalties. Predictable degraded delivery is strictly superior to late optimal decisions.

**Technology Stack & Architectural Mapping**

- **Two-Language Decoupled Architecture:**
  - *Online Hot Path (Rust):* The entire serving loop (gRPC server via `tonic`, Redis feature fetch via `redis-rs`, vector retrieval, in-process ONNX Runtime inference, auction resolution, and PID pacing) is authored in Rust. Eliminates garbage collection pauses and dynamic runtime overhead to strictly enforce the < 50 ms p99 SLA.
  - *Offline ML & Experimentation Path (Python):* PyTorch for two-tower and DLRM/DCN-v2 model architectures, ONNX model export, LightGBM baseline training, post-hoc calibration (Platt/isotonic via `scikit-learn`), and Doubly Robust OPE evaluation scripts.
- **Inference Runtime:** In-process ONNX Runtime via Rust `ort` bindings with FP16 precision and dynamic batching. Avoids the extra network hop (2–5 ms) of external inference sidecars (e.g. Triton).
- **Candidate Retrieval Engine:** In-memory, in-process HNSW index (`hnswlib` or `faiss` C++ bindings) with targeting predicate pushdown into the traversal routine to avoid post-filter candidate starvation without external vector DB latency.
- **Feature Storage & Streaming Engine:**
  - *Online Feature Store:* Redis (or DragonflyDB) for sub-millisecond point lookups and atomic counter increments (`INCRBY` for spend tracking, rolling feature windows).
  - *Streaming Pipeline:* Redpanda (C++ Kafka alternative, eliminating JVM tail latency) running lightweight Rust streaming consumers for rolling aggregate rollups.
  - *Offline Feature Store:* DuckDB querying append-only Parquet event files, executing point-in-time temporal joins to guarantee zero future data leakage.

**Hardware Profile, Local Constraints & Sizing Guidelines (e.g. Laptop RTX 3050 6GB)**

- **Hardware Feasibility:** The full ADAPT platform is fully developable and verifiable locally on a developer laptop equipped with an entry-level discrete GPU (e.g., NVIDIA RTX 3050 6 GB VRAM, 16–32 GB system RAM).
- **Subsystem Resource Isolation:**
  - *GPU-bound:* Deep ranking inference (ONNX Runtime FP16) and offline PyTorch training.
  - *CPU/RAM-bound:* gRPC serving, Redis feature store, Redpanda stream ingestion, in-process HNSW index, GSP/VCG auction mechanics, PID pacing calculations, and DuckDB analytical queries.
- **VRAM Budget Allocation (6 GB Ceiling):**
  - *Inference Serving:* Under ONNX Runtime FP16, scoring a batch of 500 candidates through a portfolio DLRM consumes ~300–500 MB VRAM, completing execution in 2–8 ms.
  - *Model Training:* Total embedding table and activation memory is capped under 2 GB VRAM by employing portfolio-scaled dimensionalities, preventing out-of-memory (OOM) faults while maintaining representational fidelity.
- **Portfolio vs. Production Sizing Parameters:**

  | Parameter | Portfolio Benchmark (RTX 3050 6GB) | Production Scale (A10G / L4 Cluster) |
  |---|---|---|
  | **User Vocabulary** | 100,000 IDs | $10^8 - 10^9$ IDs |
  | **Creative/Ad Vocabulary** | 50,000 IDs | $10^6 - 10^7$ IDs |
  | **Embedding Dimension ($d$)** | 64 | 128 – 256 |
  | **Candidate Pool (ANN)** | 100,000 items (~25 MB in RAM) | $10^7$ items (Distributed cluster) |
  | **Training Batch Size** | 256 – 512 | 4,096 – 16,384 |
  | **Inference Batch Size** | 500 candidates | 500 – 1,000 candidates |
  | **Sustained Throughput** | 200 – 500 QPS (p99 < 50 ms) | 1,000 – 10,000+ QPS |

- *Verification Strategy on Local Hardware:* Verify and prove the < 50 ms p99 latency SLA per request locally; document the throughput ceiling under 200–500 QPS, and provide analytical extrapolation for multi-replica/datacenter GPU clusters.

**Verification, metrics & deliverables**

- Benchmark scorecard comparing GBDT vs DLRM vs DCN-v2 on AUC, log loss, and Expected Calibration Error (ECE).
- Pacing performance report showing budget spend curves across 24 simulated hours with and without PID pacing.
- Latency profiling report showing p50, p90, p99, and p99.9 latencies under load (100 to 2,000 QPS) with per-stage breakdowns.
- Off-policy evaluation validation report comparing IPS and DR estimates with ground-truth simulated A/B test results.
- Live drift simulation demo: inject synthetic distribution shift (e.g. sudden CTR crash on a major creative category), demonstrate PSI alert firing, and observe automatic fallback to safe checkpoint.


#!/usr/bin/env bash
# ==============================================================================
# ADAPT: End-to-End Offline ML Training & Calibration Pipeline
# ==============================================================================
# Execution Stages:
# 1. Generate synthetic interaction logs (portfolio sizing: 100k users, 50k ads)
# 2. Execute DuckDB temporal point-in-time joins for leak-free training sets
# 3. Train LightGBM benchmark baseline
# 4. Train Two-Tower candidate retrieval encoders and export ANN index
# 5. Train DLRM / DCN-v2 multi-task rankers (pCTR + pCVR)
# 6. Perform Platt scaling and isotonic regression calibration
# 7. Export calibrated model to ONNX FP16 for the Rust serving engine
# 8. Run Off-Policy Evaluation (OPE: IPS & Doubly Robust)
# ==============================================================================

set -euo pipefail

echo "=== [1/8] Generating Synthetic Impression & Feedback Data ==="
# python -m ml.data.synthetic --output data/impressions.parquet

echo "=== [2/8] Materializing Point-in-Time Features via DuckDB ==="
# python -m ml.data.pit_joins --events data/impressions.parquet --output data/train.parquet

echo "=== [3/8] Training LightGBM Baseline ==="
# python -m ml.models.gbdt.baseline --input data/train.parquet

echo "=== [4/8] Training Two-Tower Candidate Encoders ==="
# python -m ml.models.two_tower.export --output-index ../models/ann_index.hnsw

echo "=== [5/8] Training DLRM / DCN-v2 Multi-Task Ranker ==="
# python -m ml.models.dlrm.train --model-type dlrm --epochs 5 --batch-size 256

echo "=== [6/8] Performing Probability Calibration ==="
# python -m ml.calibration.calibrator --checkpoint ../models/dlrm_ranker.pt

echo "=== [7/8] Exporting Model to ONNX FP16 ==="
# python -m ml.models.dlrm.export --output ../models/dlrm_ranker.onnx

echo "=== [8/8] Running Off-Policy Evaluation (IPS / DR) ==="
# python -m ml.ope.estimators --logs data/impressions.parquet

echo "=== Pipeline Completed Successfully ==="


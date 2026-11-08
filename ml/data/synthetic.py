"""Synthetic Data Generator for ADAPT.

Generates realistic ad impression, click, and conversion logs for local training
and benchmarking within laptop hardware constraints (RTX 3050 6GB).

Portfolio Sizing Constraints:
- User Vocabulary: 100,000 distinct user IDs
- Ad / Creative Vocabulary: 50,000 distinct ad IDs
- Publisher Domains: 500 domains
- Impressions: 1,000,000 synthetic interaction rows
- Output format: Compressed Parquet files for DuckDB point-in-time joins
"""

# TODO: Implement generate_synthetic_dataset(num_users=100000, num_ads=50000, num_rows=1000000, seed=42)
# - Generate dense numerical features (historical CTR, spend velocity, user engagement ratios)
# - Generate sparse categorical IDs (user_id, ad_id, domain_id, device_type, geo)
# - Simulate ground-truth non-linear click and conversion probabilities with realistic class imbalance (~2% CTR, ~0.2% CVR)
# - Add monotonic event timestamps across a simulated 7-day period

# TODO: Implement save_dataset_parquet(df, output_path)


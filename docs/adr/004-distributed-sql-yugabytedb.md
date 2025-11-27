# ADR-004: Distributed SQL with YugabyteDB

**Status:** Accepted
**Date:** 2025-11-27
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

Production stack templates need a distributed SQL database recommendation. The options considered:

| Database | Protocol | License | Postgres Compat |
|----------|----------|---------|-----------------|
| YugabyteDB | PostgreSQL | Apache 2.0 | 85% |
| TiDB | MySQL | Apache 2.0 | N/A |
| CockroachDB | PostgreSQL-ish | BSL (proprietary) | 53% |
| PlanetScale | MySQL | Paid service | N/A |

### Key Requirements

1. **FOSS**: Must be truly open source (Apache 2.0 or similar)
2. **PostgreSQL-compatible**: Works with Rust ecosystem (`sqlx`, `diesel`, `sea-orm`)
3. **Distributed**: Automatic sharding, rebalancing, multi-region
4. **Production-proven**: Battle-tested at scale

## Decision

**YugabyteDB is the recommended distributed SQL database for `--type rust-axum` and `--type rust-grpc` templates.**

### Why YugabyteDB

1. **100% Open Source**: Apache 2.0 license, no BSL restrictions
2. **85% PostgreSQL Compatibility**: Per PostgreSQL Compatibility Index benchmark
3. **Works with sqlx**: Standard PostgreSQL wire protocol
4. **Distributed by Design**: Inspired by Google Spanner architecture
5. **OLTP Focus**: Optimized for transactional workloads

### Why NOT CockroachDB

1. **Proprietary License**: Changed to BSL (Business Source License) - not FOSS
2. **53% PostgreSQL Compatibility**: Significantly lower than YugabyteDB's 85%
3. **License Restrictions**: Cannot use for competing database services

### Why NOT PlanetScale

1. **Free Tier Removed**: April 2024
2. **Scaling Limitations**: Manual scaling on non-enterprise plans
3. **Cost**: $3,000/month minimum for multi-region
4. **MySQL Only**: Doesn't match PostgreSQL-centric Rust ecosystem

### Why NOT TiDB

1. **MySQL Protocol**: Would require different Rust crates
2. **HTAP Focus**: Optimized for analytics + OLTP hybrid
3. **Complexity**: More complex operational model for pure OLTP

## Consequences

### Positive

- FOSS-only stack (no license surprises)
- Standard PostgreSQL tooling works
- Scales horizontally without code changes
- Multi-region capability built-in

### Negative

- Less name recognition than CockroachDB
- Smaller community than PostgreSQL
- Requires cluster management (or YugabyteDB Managed)

### Neutral

- Performance comparable to CockroachDB for most workloads
- Cloud-managed options available (YugabyteDB Managed)

## Implementation

Templates will include:

```yaml
quality:
  database:
    type: yugabytedb
    why: "FOSS (Apache 2.0), 85% Postgres compatibility, distributed"
    connection: "postgresql://..."  # Standard Postgres driver
```

## References

- [YugabyteDB vs CockroachDB](https://www.yugabyte.com/yugabytedb-vs-cockroachdb/) - Compatibility benchmark
- [CockroachDB vs Postgres 2025](https://www.bytebase.com/blog/cockroachdb-vs-postgres/) - License changes
- [PostgreSQL Compatibility Index](https://www.yugabyte.com/postgresql-compatibility/) - Benchmark methodology

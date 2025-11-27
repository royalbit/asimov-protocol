# ADR-005: Event-Driven Architecture with Redis Streams

**Status:** Accepted
**Date:** 2025-11-27
**Authors:** Claude (Opus 4.5) - Principal Engineer

## Context

Production stack templates need an event-driven messaging recommendation. The options considered:

| Component | Type | License | Complexity |
|-----------|------|---------|------------|
| Redis Streams | Stream + Consumer Groups | BSD-3 | Low |
| NATS | Pub/Sub + JetStream | Apache 2.0 | Low |
| Apache Kafka | Distributed Log | Apache 2.0 | High |
| RabbitMQ | Message Queue | MPL 2.0 | Medium |

### Key Requirements

1. **FOSS**: Must be truly open source
2. **Consumer Groups**: Multiple consumers with acknowledgment
3. **Exactly-Once Possible**: At-least-once with idempotency
4. **Rust Support**: Good async crate ecosystem
5. **Operational Simplicity**: Easy to deploy and manage

## Decision

**Redis Streams is the recommended event-driven component for `--type rust-axum` and `--type rust-grpc` templates.**

### Why Redis Streams

1. **Already in Stack**: Most services use Redis for caching anyway
2. **Consumer Groups**: Built-in support with acknowledgment (`XREADGROUP`, `XACK`)
3. **Simple Operations**: One less system to manage
4. **FOSS**: BSD-3 license
5. **Rust Support**: Excellent `redis-rs` crate with async support
6. **Persistence**: AOF/RDB for durability

### Consumer Group Pattern

```rust
// Rust with redis-rs
let mut con = client.get_async_connection().await?;

// Create consumer group
redis::cmd("XGROUP")
    .arg("CREATE")
    .arg("orders")
    .arg("order-processors")
    .arg("0")
    .arg("MKSTREAM")
    .query_async(&mut con).await?;

// Read with consumer group
let results: StreamReadReply = redis::cmd("XREADGROUP")
    .arg("GROUP").arg("order-processors").arg("worker-1")
    .arg("COUNT").arg(10)
    .arg("STREAMS").arg("orders").arg(">")
    .query_async(&mut con).await?;

// Acknowledge processed
redis::cmd("XACK")
    .arg("orders")
    .arg("order-processors")
    .arg(&message_id)
    .query_async(&mut con).await?;
```

### When to Use Kafka Instead

Redis Streams is NOT recommended when:

- **>100K events/second** sustained throughput required
- **Multi-datacenter replication** is mandatory
- **Long retention** (weeks/months) of event history
- **Existing Kafka expertise** on team

For these cases, use Apache Kafka with `rdkafka` crate.

### Why NOT NATS

1. **Additional System**: Another service to operate
2. **Less Familiar**: Smaller ecosystem than Redis
3. **JetStream Complexity**: Persistence adds operational burden

Good choice if Redis isn't already in stack.

### Why NOT RabbitMQ

1. **Queue vs Stream**: Different semantics (consume-once vs replay)
2. **Erlang Operations**: Different operational expertise needed
3. **Less Cloud-Native**: Clustering more complex

## Consequences

### Positive

- No additional infrastructure (Redis already present)
- Simple consumer group semantics
- Good Rust crate support (`redis-rs`)
- Easy local development

### Negative

- Not designed for massive scale (>100K/sec)
- Single-node bottleneck possible
- Less sophisticated than Kafka for analytics

### Neutral

- Persistence via AOF (configurable durability)
- Can migrate to Kafka later if needed

## Implementation

Templates will include:

```yaml
quality:
  events:
    type: redis-streams
    why: "Consumer groups, already in stack, FOSS (BSD-3)"
    pattern: "XREADGROUP + XACK for exactly-once processing"
```

## References

- [Redis Streams Documentation](https://redis.io/docs/data-types/streams/)
- [redis-rs crate](https://crates.io/crates/redis) - Rust async client
- [Consumer Groups Tutorial](https://redis.io/docs/data-types/streams-tutorial/)

# 🦀 Rafka — A Distributed Commit Log in Rust

Rafka is a distributed log system inspired by Apache Kafka, built from first principles in Rust.  
It focuses on durability, partitioning, replication, and high-throughput message streaming.

## Goals
- Build a durable append-only log
- Support partitions and offsets
- Support replication (eventually)
- Implement a simple producer & consumer model
- Understand internals: segment files, indexes, message formats, commit logs, and leader election

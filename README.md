# FluxDrop

High-performance eBPF/XDP network telemetry and packet enforcement engine built in Rust.

FluxDrop operates at the XDP ingress path for ultra-low-overhead packet inspection, flow tracking, telemetry generation, and selective packet dropping before traffic enters the kernel networking stack.

## Overview

FluxDrop is designed as a kernel-first observability engine focused on extracting real-time network intelligence directly from live traffic.

Unlike traditional userspace packet analyzers, FluxDrop leverages XDP for early packet interception, enabling high-speed telemetry collection and policy enforcement with minimal latency.

The engine aims to combine:

* high-throughput packet parsing
* per-flow telemetry aggregation
* TCP behavioral analysis
* packet drop enforcement
* live userspace observability

into a unified low-level telemetry system.

---

## Core Architecture

FluxDrop is organized as a Rust workspace:

```text
fluxdrop/
├── ebpf/       # XDP programs, maps, packet parser
├── userspace/  # Aya loader, map readers, event streaming
├── common/     # Shared structs and protocol types
└── dashboard/  # Live observability dashboard
```

---

## Current Goals

### Packet Processing

* Parse Ethernet headers
* Parse IPv4 packets
* Parse TCP and UDP headers
* Validate packet boundaries safely inside eBPF verifier constraints

---

### Flow Tracking

Extract and maintain per-flow state using 5-tuple keys:

* Source IP
* Destination IP
* Source Port
* Destination Port
* Protocol

Track:

* packet counts
* byte counts
* flow lifetimes
* bandwidth usage

---

### TCP Telemetry

* Infer SYN → SYN-ACK RTT
* Detect retransmissions
* Measure per-flow latency behavior
* Track connection establishment patterns

---

### Observability

Export kernel telemetry to userspace:

* flow creation events
* high-bandwidth alerts
* retransmit spikes
* protocol distributions
* packet drop events

Build a live terminal dashboard for:

* top bandwidth consumers
* top talkers
* protocol usage breakdown
* packet rate heatmaps
* drop metrics

---

### Enforcement

Policy engine (planned):

* selective packet drops
* per-IP rate limiting
* SYN flood mitigation
* port scan detection
* anomaly-triggered filtering

---

## Planned Features

* Per-flow packet counters
* Per-flow byte counters
* Per-flow bandwidth measurement
* TCP RTT estimation
* Retransmission tracking
* Protocol distribution metrics
* Top bandwidth flow detection
* Per-IP packet rate tracking
* Userspace perf/ring buffer event streaming
* Real-time TUI dashboard
* Packet drop policies
* Rate limiting
* Basic anomaly detection

---

## Tech Stack

* Rust
* Aya
* eBPF
* XDP
* Linux networking
* Tokio
* bpftool
* perf
* Prometheus (planned)
* Grafana (planned)

---

## Why FluxDrop?

Traditional observability tools often inspect traffic after it traverses the kernel stack.

FluxDrop shifts visibility and enforcement earlier — directly at ingress.

This enables:

* lower overhead telemetry
* faster anomaly detection
* earlier packet rejection
* higher scalability under load

---

## Long-Term Vision

FluxDrop aims to evolve into a lightweight programmable network observability and enforcement layer capable of:

* flow intelligence
* attack detection
* traffic shaping
* malicious packet dropping
* live telemetry export
* distributed observability integration

with near line-rate performance.

---

## Status

🚧 Active development (v1)

Current focus:

* packet parsing
* flow map design
* telemetry pipeline
* userspace event export

### Ingestion Service for SIH25022
---
## Overview
This service makes classification and manages real-time data and batch-data and accordingly send it to the appropriate service.
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Railway       │───>│  Data Ingestion  │───>│      Redis      │
│   Data Sources  │    │     Service      │    │   (Real-time    │
└─────────────────┘    └──────────────────┘    │    Streams)     │
                                │              └─────────────────┘
                                v                       │
                       ┌─────────────────┐              │
                       │ Queue Processor │              │
                       │     Kafka       │              │
                       │  (Batch Data)   │              │
                       └─────────────────┘              │
                                │                       │
                                v                       │
                       ┌─────────────────┐              │
                       │   Data Lake +   │              │
                       │ Time Series DB  │              │
                       └─────────────────┘              │
                                │                       │
                                └────────┬──────────────┘
                                         v
                                ┌─────────────────┐
                                │  Event Bus      │
                                │     Kafka       │
                                │ (Event Manager) │
                                └─────────────────┘
                                         │
                                         v
                                ┌─────────────────┐
                                │   Downstream    │
                                │   Consumers     │
                                │ (Analytics,     │
                                │  ML, Alerts)    │
                                └─────────────────┘
```
## Tech Stack Used
- Rust
- Kafka
- Redis
## Frameworks
- Axum
- tokio
- redis
- rdkafka 
- tower*

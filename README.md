# Metric Analyser

## Overview
Metric Analyser is a Rust-based application designed to analyze metrics and send them to a metric service via Kafka. It utilizes a metric engine to collect and process metrics, then communicates with a Kafka cluster to transmit the processed data to a designated metric service.

## Features
- Metric Engine: Collects and processes metrics data.
- Kafka Integration: Communicates with Kafka to send processed metrics.
- Configurability: Supports configuration for Kafka endpoints, metric service details, and other parameters.

#!/bin/zsh

grpcurl -plaintext \
  -import-path proto \
  -proto proto/ingest.proto \
  -d @ 127.0.0.1:50051 ecg.ingest.v1.Ingestor/IngestOnce <<'EOF'
{
  "sessionId": "test-stream",
  "fsHz": 360.0,
  "samples": [
    { "seq": 1, "tS": 0.000, "mv": 0.10, "status": 1 },
    { "seq": 2, "tS": 0.003, "mv": 0.12, "status": 1 },
    { "seq": 3, "tS": 0.006, "mv": 0.09, "status": 1 },
    { "seq": 4, "tS": 0.009, "mv": 0.15, "status": 1 },
    { "seq": 5, "tS": 0.012, "mv": 0.11, "status": 1 },
    { "seq": 6, "tS": 0.015, "mv": 0.14, "status": 1 },
    { "seq": 7, "tS": 0.018, "mv": 0.13, "status": 1 },
    { "seq": 8, "tS": 0.021, "mv": 0.16, "status": 1 },
    { "seq": 9, "tS": 0.024, "mv": 0.10, "status": 1 },
    { "seq": 10, "tS": 0.027, "mv": 0.12, "status": 1 }
  ]
}
EOF

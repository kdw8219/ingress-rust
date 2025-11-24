#!/bin/sh
set -e

# Defaults (can be overridden by environment variables passed from docker-compose)
: "${KAFKA_BOOTSTRAP:=kafka-broker:9092}"
: "${TSDB_HOST:=timescaledb}"
: "${TSDB_PORT:=5435}"
: "${TSDB_USER:=dk}"
: "${TSDB_PASSWORD:=1234}"
: "${TSDB_DBNAME:=robot_stat_db}"

mkdir -p /app/config

cat > /app/config/default.toml <<EOF
[kafka]
bootstrap_servers = "${KAFKA_BOOTSTRAP}"
group_id = "robot-ingress"
topics = ["robot-heartbeat", "robot-status", "robot-position"]
auto_offset_reset = "earliest"

[timeseries]
host = "${TSDB_HOST}"
port = "${TSDB_PORT}"
user = "${TSDB_USER}"
password = "${TSDB_PASSWORD}"
dbname = "${TSDB_DBNAME}"
EOF

exec /usr/local/bin/ingress-rust

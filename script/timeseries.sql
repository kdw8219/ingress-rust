CREATE TABLE robot_heartbeat (
    robot_id     TEXT NOT NULL,
    is_alive     BOOLEAN NOT NULL,
    ts           TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (robot_id, ts)
);

SELECT create_hypertable('robot_heartbeat', 'ts', if_not_exists => TRUE);

SELECT add_retention_policy('robot_heartbeat', INTERVAL '10 days');

ALTER TABLE robot_heartbeat SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'robot_id'
);

SELECT add_compression_policy('robot_heartbeat', INTERVAL '7 days');

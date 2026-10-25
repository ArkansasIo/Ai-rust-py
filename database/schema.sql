CREATE DATABASE IF NOT EXISTS ai_ml;
USE ai_ml;

CREATE TABLE IF NOT EXISTS datasets (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  task_type ENUM('classification','regression','clustering','reinforcement') NOT NULL,
  path VARCHAR(1024),
  row_count BIGINT DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS models (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  model_type VARCHAR(100) NOT NULL,
  framework VARCHAR(100),
  version VARCHAR(64) NOT NULL,
  artifact_path VARCHAR(1024),
  status ENUM('created','training','ready','failed','retired') DEFAULT 'created',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS training_runs (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  model_id BIGINT NOT NULL,
  dataset_id BIGINT,
  status ENUM('queued','running','completed','failed') DEFAULT 'queued',
  metrics_json JSON,
  started_at TIMESTAMP NULL,
  finished_at TIMESTAMP NULL,
  FOREIGN KEY (model_id) REFERENCES models(id),
  FOREIGN KEY (dataset_id) REFERENCES datasets(id)
);

CREATE TABLE IF NOT EXISTS predictions (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  model_id BIGINT NOT NULL,
  input_json JSON NOT NULL,
  output_json JSON NOT NULL,
  latency_ms DOUBLE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (model_id) REFERENCES models(id)
);

CREATE TABLE IF NOT EXISTS ai_events (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  event_type VARCHAR(100) NOT NULL,
  payload_json JSON,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

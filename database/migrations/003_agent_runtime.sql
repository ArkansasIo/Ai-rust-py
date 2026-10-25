CREATE TABLE IF NOT EXISTS agent_tasks (
  id CHAR(36) PRIMARY KEY,
  kind VARCHAR(120) NOT NULL,
  payload_json JSON NOT NULL,
  status ENUM('queued','running','success','failed','paused','cancelled') NOT NULL DEFAULT 'queued',
  attempts INT NOT NULL DEFAULT 0,
  worker_id VARCHAR(120),
  result_json JSON,
  error_message TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  started_at TIMESTAMP NULL,
  finished_at TIMESTAMP NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_agent_tasks_status (status),
  INDEX idx_agent_tasks_created (created_at)
);

CREATE TABLE IF NOT EXISTS workflow_checkpoints (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  workflow_run_id CHAR(36) NOT NULL,
  node_id VARCHAR(120) NOT NULL,
  state_json JSON NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_checkpoint_run (workflow_run_id),
  INDEX idx_checkpoint_node (node_id)
);

CREATE TABLE IF NOT EXISTS tool_approvals (
  id CHAR(36) PRIMARY KEY,
  task_id CHAR(36),
  tool_name VARCHAR(120) NOT NULL,
  risk_level ENUM('read','low','medium','high','critical') NOT NULL,
  requested_by VARCHAR(255),
  approved_by VARCHAR(255),
  status ENUM('pending','approved','denied','expired') NOT NULL DEFAULT 'pending',
  reason TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  resolved_at TIMESTAMP NULL,
  INDEX idx_tool_approvals_status (status),
  INDEX idx_tool_approvals_task (task_id)
);

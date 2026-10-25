CREATE TABLE IF NOT EXISTS users (
  id CHAR(36) PRIMARY KEY,
  username VARCHAR(120) NOT NULL UNIQUE,
  role ENUM('user','developer','operator','admin') NOT NULL DEFAULT 'user',
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agents (
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(160) NOT NULL UNIQUE,
  class_name VARCHAR(80) NOT NULL,
  status VARCHAR(32) NOT NULL DEFAULT 'created',
  model_name VARCHAR(255) NOT NULL,
  system_prompt TEXT,
  temperature DECIMAL(4,3) NOT NULL DEFAULT 0.7,
  max_steps INT NOT NULL DEFAULT 16,
  config_json JSON,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_agents_status(status)
);

CREATE TABLE IF NOT EXISTS conversations (
  id CHAR(36) PRIMARY KEY,
  user_id CHAR(36),
  title VARCHAR(255) NOT NULL,
  system_prompt TEXT,
  summary TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
  INDEX idx_conversations_user(user_id)
);

CREATE TABLE IF NOT EXISTS messages (
  id CHAR(36) PRIMARY KEY,
  conversation_id CHAR(36) NOT NULL,
  role_name ENUM('system','user','assistant','tool') NOT NULL,
  content LONGTEXT NOT NULL,
  model_name VARCHAR(255),
  tool_call_id CHAR(36),
  prompt_tokens BIGINT DEFAULT 0,
  completion_tokens BIGINT DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE,
  INDEX idx_messages_conversation_created(conversation_id, created_at)
);

CREATE TABLE IF NOT EXISTS memories (
  id CHAR(36) PRIMARY KEY,
  user_id CHAR(36),
  memory_type ENUM('semantic','episodic','procedural','preference','summary') NOT NULL DEFAULT 'semantic',
  content LONGTEXT NOT NULL,
  embedding_json JSON,
  importance DECIMAL(5,4) NOT NULL DEFAULT 0.5,
  metadata_json JSON,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  accessed_at TIMESTAMP NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
  INDEX idx_memories_user_type(user_id, memory_type),
  INDEX idx_memories_importance(importance)
);

CREATE TABLE IF NOT EXISTS tools (
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(160) NOT NULL UNIQUE,
  description TEXT NOT NULL,
  kind VARCHAR(64) NOT NULL,
  risk_level VARCHAR(32) NOT NULL DEFAULT 'read_only',
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  input_schema JSON NOT NULL,
  output_schema JSON,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tool_calls (
  id CHAR(36) PRIMARY KEY,
  tool_id CHAR(36) NOT NULL,
  conversation_id CHAR(36),
  input_json JSON NOT NULL,
  output_json JSON,
  approved BOOLEAN NOT NULL DEFAULT FALSE,
  status VARCHAR(32) NOT NULL DEFAULT 'queued',
  started_at TIMESTAMP NULL,
  finished_at TIMESTAMP NULL,
  FOREIGN KEY (tool_id) REFERENCES tools(id),
  FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE SET NULL,
  INDEX idx_tool_calls_status(status)
);

CREATE TABLE IF NOT EXISTS workflows (
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(160) NOT NULL,
  version INT NOT NULL DEFAULT 1,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  definition_json JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_workflow_version(name, version)
);

CREATE TABLE IF NOT EXISTS workflow_runs (
  id CHAR(36) PRIMARY KEY,
  workflow_id CHAR(36) NOT NULL,
  status VARCHAR(32) NOT NULL DEFAULT 'queued',
  input_json JSON,
  output_json JSON,
  error_text TEXT,
  started_at TIMESTAMP NULL,
  finished_at TIMESTAMP NULL,
  FOREIGN KEY (workflow_id) REFERENCES workflows(id),
  INDEX idx_workflow_runs_status(status)
);

CREATE TABLE IF NOT EXISTS ai_tasks (
  id CHAR(36) PRIMARY KEY,
  task_name VARCHAR(160) NOT NULL,
  priority ENUM('low','normal','high','critical') NOT NULL DEFAULT 'normal',
  status ENUM('queued','running','completed','failed','cancelled') NOT NULL DEFAULT 'queued',
  payload_json JSON NOT NULL,
  result_json JSON,
  attempts INT NOT NULL DEFAULT 0,
  max_attempts INT NOT NULL DEFAULT 3,
  available_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  completed_at TIMESTAMP NULL,
  INDEX idx_ai_tasks_queue(status, priority, available_at)
);

CREATE TABLE IF NOT EXISTS audit_events (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  actor_id CHAR(36),
  event_type VARCHAR(120) NOT NULL,
  resource_type VARCHAR(120),
  resource_id VARCHAR(120),
  payload_json JSON,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_audit_created(created_at),
  INDEX idx_audit_type(event_type)
);

CREATE TABLE IF NOT EXISTS system_jobs (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  job_name VARCHAR(160) NOT NULL UNIQUE,
  last_run_at TIMESTAMP NULL,
  last_status VARCHAR(32),
  last_error TEXT,
  run_count BIGINT NOT NULL DEFAULT 0,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

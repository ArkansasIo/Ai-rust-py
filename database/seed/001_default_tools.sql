USE ai_ml;

INSERT IGNORE INTO tools (id, name, description, kind, risk_level, enabled, input_schema, output_schema) VALUES
(UUID(), 'memory_search', 'Search persistent AI memory.', 'database', 'read_only', TRUE, JSON_OBJECT('type','object','properties',JSON_OBJECT('query',JSON_OBJECT('type','string'))), JSON_OBJECT('type','array')),
(UUID(), 'http_get', 'Fetch an explicitly requested HTTP resource.', 'http', 'medium', TRUE, JSON_OBJECT('type','object','properties',JSON_OBJECT('url',JSON_OBJECT('type','string'))), JSON_OBJECT('type','object')),
(UUID(), 'python_execute', 'Execute approved Python analysis code in a restricted worker.', 'python', 'high', FALSE, JSON_OBJECT('type','object'), JSON_OBJECT('type','object')),
(UUID(), 'shell_execute', 'Execute an approved shell command in a restricted worker.', 'shell', 'critical', FALSE, JSON_OBJECT('type','object'), JSON_OBJECT('type','object'));

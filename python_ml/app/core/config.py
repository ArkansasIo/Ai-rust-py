import os
from dataclasses import dataclass

@dataclass(frozen=True)
class Settings:
    app_name: str = os.getenv('APP_NAME', 'Ai-rust-py ML Service')
    environment: str = os.getenv('ENVIRONMENT', 'development')
    ollama_url: str = os.getenv('OLLAMA_URL', 'http://ollama:11434')
    ollama_model: str = os.getenv('OLLAMA_MODEL', 'llama3.2')
    embedding_model: str = os.getenv('EMBEDDING_MODEL', 'nomic-embed-text')
    rust_api_url: str = os.getenv('RUST_API_URL', 'http://rust-api:8080')
    mysql_host: str = os.getenv('MYSQL_HOST', 'mysql')
    mysql_port: int = int(os.getenv('MYSQL_PORT', '3306'))
    mysql_database: str = os.getenv('MYSQL_DATABASE', 'ai_ml')
    mysql_user: str = os.getenv('MYSQL_USER', 'ai')
    mysql_password: str = os.getenv('MYSQL_PASSWORD', 'change-me')
    max_context_tokens: int = int(os.getenv('MAX_CONTEXT_TOKENS', '8192'))
    max_tool_steps: int = int(os.getenv('MAX_TOOL_STEPS', '16'))

settings = Settings()

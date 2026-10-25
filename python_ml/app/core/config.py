from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file='.env', extra='ignore')
    app_name: str = 'Ai-rust-py ML Service'
    environment: str = 'development'
    ollama_url: str = 'http://ollama:11434'
    ollama_model: str = 'llama3.2'
    embedding_model: str = 'nomic-embed-text'
    rust_api_url: str = 'http://rust-api:8080'
    mysql_host: str = 'mysql'
    mysql_port: int = 3306
    mysql_database: str = 'ai_ml'
    mysql_user: str = 'ai'
    mysql_password: str = 'change-me'
    max_context_tokens: int = 8192
    max_tool_steps: int = 16

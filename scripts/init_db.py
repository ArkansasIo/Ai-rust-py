import os
import pymysql

host = os.getenv("MYSQL_HOST", "127.0.0.1")
port = int(os.getenv("MYSQL_PORT", "3306"))
user = os.getenv("MYSQL_USER", "ai_user")
password = os.getenv("MYSQL_PASSWORD", "ai_password")
database = os.getenv("MYSQL_DATABASE", "ai_ml")

with open("database/schema.sql", "r", encoding="utf-8") as f:
    sql = f.read()

connection = pymysql.connect(host=host, port=port, user=user, password=password, database=database, autocommit=True)
try:
    with connection.cursor() as cursor:
        for statement in sql.split(";"):
            statement = statement.strip()
            if statement:
                cursor.execute(statement)
finally:
    connection.close()

print("Database initialized.")

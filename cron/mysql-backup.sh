#!/usr/bin/env sh
set -eu

BACKUP_DIR="${BACKUP_DIR:-./backups}"
DB_HOST="${MYSQL_HOST:-127.0.0.1}"
DB_PORT="${MYSQL_PORT:-3306}"
DB_NAME="${MYSQL_DATABASE:-ai_ml}"
DB_USER="${MYSQL_USER:-ai}"
mkdir -p "$BACKUP_DIR"
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
OUT="$BACKUP_DIR/${DB_NAME}-${STAMP}.sql"

: "${MYSQL_PASSWORD:?MYSQL_PASSWORD must be set}"
mysqldump --host="$DB_HOST" --port="$DB_PORT" --user="$DB_USER" --password="$MYSQL_PASSWORD" --single-transaction --routines --triggers "$DB_NAME" > "$OUT"

gzip -f "$OUT"
find "$BACKUP_DIR" -type f -name '*.sql.gz' -mtime +14 -delete

#!/usr/bin/env bash
set -euo pipefail

IMG="ghcr.io/mhsanaei/3x-ui:v2.6.6"
NAME="xui"
BASE="http://127.0.0.1:2053"
USER="admin"
PASS="admin"

cleanup() { docker rm -f "$NAME" >/dev/null 2>&1 || true; }
trap cleanup EXIT

docker pull "$IMG"
docker rm -f "$NAME" >/dev/null 2>&1 || true

# РЕКОМЕНДУЕМО по докам — host-network; так порт 2053 будет доступен напрямую
docker run -d --name "$NAME" --network host "$IMG"

# Ожидаем поднятия панели и работоспособности /login (доки: логин admin/admin)
for i in $(seq 1 90); do
  code=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H 'Content-Type: application/json' \
    -d "{\"username\":\"${USER}\",\"password\":\"${PASS}\"}" \
    "${BASE}/login" || true)
  if [ "$code" = "200" ]; then
    echo "3x-ui is ready"
    break
  fi
  sleep 2
  if [ "$i" = "90" ]; then
    echo "3x-ui failed to become ready"
    docker logs "$NAME" || true
    exit 1
  fi
done

export PANEL_BASE_URL="${BASE}/"
export PANEL_USERNAME="${USER}"
export PANEL_PASSWORD="${PASS}"
export RUST_LOG="trace"

cargo test --tests -- --nocapture

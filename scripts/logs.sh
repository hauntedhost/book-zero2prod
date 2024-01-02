#!/usr/bin/env bash
# simple one-line log format parsed from json logs
#
# Usage:
#   cargo run | tee -a logs/dev.log (or ./dev.sh)
#   ./logs.sh

logfile="${1:-logs/dev.log}"
tail -f $logfile | \
  jq -r '"\(.time)\t\(.["otel.name"] // .msg)\t\(.request_id // "∅")"'

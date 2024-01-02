#!/usr/bin/env bash
# start cargo with dev log

cargo run | tee -a logs/dev.log

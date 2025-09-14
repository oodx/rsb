#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[E2E] pre_dispatch on 'init' with --verbose"
out1=$(cargo run --quiet --example cli_e2e -- init --verbose || true)
echo "$out1"
grep -q "pre:init" <<<"$out1"
grep -q "opt_verbose=1" <<<"$out1"

echo "[E2E] dispatch on 'run' with --config=foo.conf"
out2=$(cargo run --quiet --example cli_e2e -- run --config=foo.conf || true)
echo "$out2"
grep -q "dispatch:run" <<<"$out2"
grep -q "opt_config=foo.conf" <<<"$out2"

echo "OK"


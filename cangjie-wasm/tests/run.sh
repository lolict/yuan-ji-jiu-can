#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")/.."
mkdir -p build

CJC=${CJC:-$(command -v cjc || echo /root/.local/bin/cjc)}
if ! command -v wasmtime >/dev/null 2>&1; then
    echo "wasmtime not found" >&2
    exit 1
fi

fail=0

check() {
    local name=$1
    local want=$2

    (cd build && "$CJC" --dump-chir --dump-to-screen "../tests/$name.cj" > "$name.chirtxt" 2>/dev/null)
    ./chir_wasm "build/$name.chirtxt" "build/$name.wasm"
    local got
    got=$(wasmtime run --invoke main "build/$name.wasm" 2>/dev/null)

    if [ "$got" = "$want" ]; then
        echo "ok   $name: $got"
    else
        echo "FAIL $name: got $got, want $want" >&2
        fail=1
    fi
}

check add 7
check add2 0
check fact 120
check loop 45
check loop2 10
check brk 6
check unit 42
check sumrange 55
check cont 25
check boolret 1
check nested 9

exit $fail

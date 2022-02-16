#!/bin/bash
set -eu

pushd $(dirname $0) &> /dev/null

cargo build --target x86_64-unknown-uefi
uefi-run target/x86_64-unknown-uefi/debug/test-runner.efi

popd &> /dev/null

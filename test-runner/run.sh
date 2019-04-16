#!/bin/bash

pushd $(dirname $0) &> /dev/null

cargo xbuild --target x86_64-unknown-uefi
uefi-run target/x86_64-unknown-uefi/debug/test-runner.efi

popd &> /dev/null

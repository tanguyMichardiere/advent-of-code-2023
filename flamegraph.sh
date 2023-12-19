#!/usr/bin/env sh

CARGO_PROFILE_RELEASE_DEBUG=true \
    cargo flamegraph --root
open flamegraph.svg

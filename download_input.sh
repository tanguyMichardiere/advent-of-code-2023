#!/usr/bin/env sh

DAY=$(date "+%d")
aoc download --year 2023 --day "$DAY" --input-only --input-file "inputs/$DAY"

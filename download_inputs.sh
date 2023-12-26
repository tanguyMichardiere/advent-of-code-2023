#!/usr/bin/env sh

for i in $(seq -f "%02g" 1 25)
do
    aoc download --year 2023 --day "$i" --input-only --input-file "inputs/$i"
done

#!/bin/bash
cargo build

for dir in testprogs/*
do
	# echo "${dir}/main.c"
	gcc "${dir}/main.c" -o "${dir}/main"
done



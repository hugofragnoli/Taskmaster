#!/bin/bash
cargo build

for dir in testprogs/*
do
	gcc "${dir}/main.c" -o "${dir}/main"
done



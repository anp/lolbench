#!/usr/bin/env bash
taskset -c 1-7 cargo bench

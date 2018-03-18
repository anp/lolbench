#!/usr/bin/env bash

set +xeo pipefail

cleanup_shield() {
  # if we're exiting we want to free up all of our resources
  cset shield --reset
}

trap cleanup_shield EXIT SIGTERM

# sets up a cpuset "shield" by creating two cpusets and moving all running processes into the smaller one
cset shield --cpu=1-7 --kthread=on

# just print some info
cset shield

# run our benchmarks under the cpuset shield
cset sh whoami
cset sh cargo +stable bench

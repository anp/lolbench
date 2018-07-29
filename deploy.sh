#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

set -xe

playbook="$SCRIPTPATH/deploy/site.yml"
inventory="$SCRIPTPATH/deploy/hosts"

# NOTE: when setting up a new machine this has to be re-enabled
    # --ask-become-pass \
ansible-playbook \
    --inventory "$inventory" \
    "$playbook"

#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

set -xe

playbook="$SCRIPTPATH/deploy/site.yml"
inventory="$SCRIPTPATH/deploy/hosts"

ansible-playbook --ask-become-pass --inventory "$inventory" "$playbook"

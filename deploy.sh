#!/usr/bin/env bash

sha="$1"

if [[ -z $sha ]]; then
    sha="$(git rev-parse HEAD)"
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
cd "$DIR" || exit 1

ansible-playbook \
    --vault-id .ansible-vault-password \
    --extra-vars "gitsha=$sha" \
    --inventory deploy/hosts \
    deploy/site.yml

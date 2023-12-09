#!/usr/bin/env bash
set -eu

export ROOT_DIR=$(pwd)
export SCRIPT_DIR="$ROOT_DIR/scripts"

scriptName=$1
shift

common="$SCRIPT_DIR/_common"
if [ -f "$common" ]; then
    source "$common"
fi

if [ -f "$SCRIPT_DIR/$scriptName" ]; then
    "$SCRIPT_DIR/$scriptName" "$@"
else
    echo "No script called '$scriptName'"
fi

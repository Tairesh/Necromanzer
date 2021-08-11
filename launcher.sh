#!/bin/sh

TARGET_FILE=$0

cd "$(dirname "$TARGET_FILE")" || {
    echo "Could not change directory to '$TARGET_FILE'" >&2
    exit 1
}

exec ./necromanzer "$@"

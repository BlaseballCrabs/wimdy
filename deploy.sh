#!/bin/sh
set -e
trunk build --release
aws s3 sync ./dist/ "$S3_URI" --acl public-read --delete "$@"

#!/bin/sh
#
# Use this script to run your program LOCALLY.
#

set -e # Exit early if any commands fail

(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release --target-dir=/tmp/build-claude-code-diy --manifest-path Cargo.toml
)

exec /tmp/build-claude-code-diy/release/claude-code-diy "$@"

#! /bin/sh

scripts=$(dirname "$(realpath "$0")")
"$scripts/install-trunk.sh"

repo=$(dirname "$(dirname "$(realpath "$0")")")
cd "$repo" || exit 1

trunk build --release --dist public

#!/usr/bin/env bash
# Build and headlessly smoke-test every game: export each to a cartridge
# PNG and run 60 frames with `pixel8 verify`, catching carts that fail to
# build or panic at runtime. Generic over the games present.
#
#   PIXEL8=path/to/pixel8 ./.github/verify-games.sh
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
pixel8="${PIXEL8:?set PIXEL8 to the pixel8 console binary}"
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

found=0
for manifest in "$root"/*/Cargo.toml; do
  dir="$(dirname "$manifest")"
  [ -f "$dir/assets.pixel8.json" ] || continue
  name="$(basename "$dir")"
  echo "::group::verify $name"
  "$pixel8" export "$dir" "$out/$name.png"
  "$pixel8" verify "$out/$name.png"
  echo "::endgroup::"
  found=1
done

if [ "$found" -eq 0 ]; then
  echo "no games found (a game is a directory with Cargo.toml + assets.pixel8.json)" >&2
  exit 1
fi

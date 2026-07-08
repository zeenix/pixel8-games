#!/usr/bin/env bash
# Export every game to GitHub Pages: a shelf of cartridge PNGs, each
# linking to its playable single-file web export, plus a generated index.
#
# A "game" is any top-level directory holding both a Cargo.toml and an
# assets.pixel8.json, so adding a new game needs no changes here or in CI.
#
# Needs the pixel8 console binary and the pixel8-web crate (to build the
# browser player). Point at them with PIXEL8 and PIXEL8_WEB:
#
#   PIXEL8=path/to/pixel8 PIXEL8_WEB=path/to/pixel8-web \
#     ./.github/build-games-site.sh site
set -euo pipefail

site="${1:-site}"
root="$(cd "$(dirname "$0")/.." && pwd)"
pixel8="${PIXEL8:?set PIXEL8 to the pixel8 console binary}"
# export-web compiles the browser player from this crate; export the var so
# the pixel8 binary picks it up.
export PIXEL8_WEB="${PIXEL8_WEB:?set PIXEL8_WEB to the pixel8-web crate dir}"

mkdir -p "$site"

games=()
for manifest in "$root"/*/Cargo.toml; do
  dir="$(dirname "$manifest")"
  [ -f "$dir/assets.pixel8.json" ] || continue
  name="$(basename "$dir")"
  echo "::group::build $name"
  # A cartridge PNG (carries the wasm + assets + source), a headless smoke
  # test, then the standalone playable web page — the same pipeline the
  # pixel8 console ships for its example carts.
  "$pixel8" export "$dir" "$site/$name.png"
  "$pixel8" verify "$site/$name.png"
  "$pixel8" export-web "$site/$name.png" "$site/$name.html"
  echo "::endgroup::"
  games+=("$name")
done

if [ ${#games[@]} -eq 0 ]; then
  echo "no games found (a game is a directory with Cargo.toml + assets.pixel8.json)" >&2
  exit 1
fi

"$(dirname "$0")/build-index.sh" "$site" "${games[@]}"

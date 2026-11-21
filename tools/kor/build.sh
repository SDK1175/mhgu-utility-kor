#!/usr/bin/env bash
# Korean fork: everything after a change, in one go (MSYS2 UCRT64).
#
#   bash tools/kor/build.sh
#
# Takes the latest code, builds the plugin, and copies it into the port's mods folder. Paths below are this PC's;
# change them here if a folder moves.
set -euo pipefail
GU=/f/MHXX_port/gu_romfs/0100770008DD8000
DEST=/f/MHXX_port/MHXXRecomp/build/mods/hunters-compendium/romfs/nativeNX/plugins
cd "$(dirname "$0")/../.."

echo "== 1/5 latest code"
git checkout -- mod/Cargo.lock mod/src/ko_dict.rs 2>/dev/null || true   # (files the build rewrites)
git pull --ff-only

echo "== 2/5 libraries"
(cd mod && cargo +nightly fetch -q)

echo "== 3/5 Korean name keys and dictionary"
if [ ! -f build/generated/item_data.rs ]; then
  echo "first time: running tools/kor/setup.sh (slow)"; bash tools/kor/setup.sh "$GU"
else
  MHGU_DATABASE="$PWD/inputs/mhgu.db" MHGU_GAME_DIR="$GU" python3 tools/set_data.py   # (skill explanations)
  python3 tools/kor/keys.py "$GU"
  python3 tools/kor/dict.py
  git checkout -- mod/src/ko_dict.rs 2>/dev/null || true
fi

echo "== 4/5 build"
bash mod/build-local.sh > build/build.log 2>&1 || { tail -40 build/build.log; echo; echo "BUILD FAILED: send build/build.log"; exit 1; }
grep -E 'Plugin footprint|test result' build/build.log | sort | uniq -c

echo "== 5/5 copy into the game"
mkdir -p "$DEST"
# Weapon and armor names: the GU Korean patch's tables, from this PC's copy (read by the plugin at run time; never shipped)
KP=/f/MHXX_port/MHGU-KoreanPatch/romfs/nativeNX/eng/table
if [ -d "$KP" ]; then
  mkdir -p "$DEST/../eng/table"
  cp "$KP"/*.gmd "$DEST/../eng/table/"
  echo "GU Korean patch tables: $(ls "$KP"/*.gmd | wc -l) copied"
else
  echo "no GU Korean patch at $KP: weapon and armor names stay as the MHXX patch has them"
fi
cp build/hunters_compendium.nro "$DEST/"
ls -la "$DEST/hunters_compendium.nro"
echo "done: start the game, F6"

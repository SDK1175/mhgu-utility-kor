#!/usr/bin/env bash
# Korean fork: the local inputs for a build, made once (MSYS2 UCRT64 on Windows, or Linux).
#
#   bash tools/kor/setup.sh <your MHGU dump folder, the one holding romfs/>
#   bash mod/build-local.sh
#
# - MHGenDatabase (MIT): its SQLite file, from GitHub, into inputs/
# - the Kiranico catalog: tools/catalog.py, a page every 0.35 s (half an hour or more the first time; cached)
# - the artwork cache upstream keeps locally: empty folders here. The build leaves out what is missing
#   (the location illustrations and the six weapon renders); nothing else depends on them.
# - tools/prepare.py with your own MHGU files (read only; nothing of them leaves your PC)
set -euo pipefail
cd "$(dirname "$0")/../.."
GU="${1:?usage: bash tools/kor/setup.sh <MHGU dump folder holding romfs/>}"
if [ ! -d "$GU/romfs/nativeNX/eng/table" ]; then
  echo "$GU: no romfs/nativeNX/eng/table here (give the folder that holds romfs/)" >&2
  exit 1
fi
mkdir -p inputs/art/location-art inputs/art/weapon-art
if [ ! -f inputs/mhgu.db ]; then
  echo "== MHGenDatabase"
  curl -fL --retry 3 -o inputs/mhgu.db.zip \
    https://raw.githubusercontent.com/gatheringhallstudios/MHGenDatabase/develop/app/src/main/assets/databases/mhgu.db.zip
  python3 - <<'PY'
import zipfile
z = zipfile.ZipFile('inputs/mhgu.db.zip')
name = next(n for n in z.namelist() if n.endswith('.db'))
open('inputs/mhgu.db', 'wb').write(z.read(name))
PY
fi
if [ ! -f build/data/catalog.json ]; then
  echo "== Kiranico catalog (slow the first time; run again to resume)"
  python3 tools/catalog.py build
fi
echo "== generated data"
python3 tools/prepare.py --catalog build/data/catalog.json --database inputs/mhgu.db --game-dir "$GU" --art-dir inputs/art
echo "done: now bash mod/build-local.sh"

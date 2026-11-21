#!/usr/bin/env bash
# Korean fork: everything to check in one file, build/kor/diag.txt (attach it in the chat).
#   bash tools/kor/diag.sh
cd "$(dirname "$0")/../.."
B=/f/MHXX_port/MHXXRecomp/build
K=$B/mods/MHXX_NX_1.5_kor_v2/0100C3800049C000/romfs/nativeNX
D=$B/data/romfs/nativeNX
G=/f/MHXX_port/gu_romfs/0100770008DD8000/romfs/nativeNX/eng/table
R=$K/arc/resident.arc; [ -f "$R" ] || R=$D/arc/resident.arc
mkdir -p build/kor
{
  echo "== port log"; head -1 $B/log.txt; grep -a 'Korean\|set_player\|Gathering map\|not provided' $B/log.txt | head -40
  echo; echo "== weapon03 (patch _jpn.gmd)"; python3 tools/kor/peek.py $K/table/weapon03MsgData_jpn.gmd $G/weapon03MsgData_eng.gmd 0 14 "$R" 2>&1
  echo; echo "== weapon03 (game .w03m)"; python3 tools/kor/peek.py $D/table/weapon03MsgData.w03m $G/weapon03MsgData_eng.gmd 0 14 "$R" 2>&1
  echo; echo "== armor series (patch)"; python3 tools/kor/peek.py $K/table/armorSeriesData_jpn.gmd $G/armorSeriesData_eng.gmd 0 20 "$R" 2>&1
  echo; echo "== GU eng tables"; ls $G | head -100
  echo; echo "== patch arc folders"; ls $K/arc 2>&1 | head -40
  P=/f/MHXX_port/MHGU-KoreanPatch
  echo; echo "== GU Korean patch files"; find $P -type f \( -iname '*weapon03*' -o -iname '*armorSeries*' -o -iname '*itemData*' -o -iname '*.nro' \) 2>&1 | head -20
  echo; echo "== GU Korean patch: what it holds"; find $P -type f | sed "s|$P/||" | cut -d/ -f1-5 | sort | uniq -c | sort -rn | head -20
  W=$(find $P -type f -iname 'weapon03MsgData_eng.gmd' | head -1)
  if [ -n "$W" ]; then echo; echo "== GU Korean patch weapon03"; python3 tools/kor/peek.py "$W" $G/weapon03MsgData_eng.gmd 0 14 2>&1; fi
  A=$(find $P -type f -iname 'armorSeriesData_eng.gmd' | head -1)
  if [ -n "$A" ]; then echo; echo "== GU Korean patch armor"; python3 tools/kor/peek.py "$A" $G/armorSeriesData_eng.gmd 8 12 2>&1; fi
} > build/kor/diag.txt 2>&1
echo "done: attach build/kor/diag.txt"

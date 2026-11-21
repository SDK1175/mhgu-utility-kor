"""Korean fork: mod/assets/paperlogy-ko.ttf, the Hangul of Paperlogy (SIL OFL 1.1) the plugin draws with.

    pip install fonttools
    python3 tools/kor/font.py <Paperlogy-5Medium.ttf>

The 2350 KS X 1001 syllables (all a Korean patch for MHXX can write), the compatibility jamo, and any other
syllable tools/kor/ui_ko.tsv uses.
"""
import subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/kor'))
from dict import entries

codes = {ord(bytes([0xB0 + i // 94, 0xA1 + i % 94]).decode('cp949')) for i in range(2350)}
codes |= set(range(0x3131, 0x318F))
for a, b in ((0x80, 0x24F), (0x2000, 0x206F), (0x2190, 0x21FF), (0x2460, 0x24FF), (0x25A0, 0x25FF), (0x3001, 0x303F)):
    codes |= set(range(a, b + 1))  # (symbols the game's font may lack; the subset keeps those Paperlogy has)
codes |= {ord(c) for v in entries().values() for c in v if 0xAC00 <= ord(c) <= 0xD7A3}
subprocess.run(['pyftsubset', sys.argv[1], '--unicodes=' + ','.join('U+%04X' % c for c in sorted(codes)),
                '--output-file=' + str(ROOT / 'mod/assets/paperlogy-ko.ttf'), '--no-hinting', '--desubroutinize'],
               check=True)
print(len(codes), 'code points')

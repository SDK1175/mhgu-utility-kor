"""Korean fork, diagnosis: an MHXX GMD next to the MHGU one, string by string (prints to your terminal only).

    python3 tools/kor/peek.py <MHXX gmd> <MHGU _eng.gmd> [first index] [count] [resident.arc]

With resident.arc (the one the game reads: the patch's if it has one), the patch's codes are read from its font
(font_festa: the 2350 glyphs from the one at U+7A00, in KS X 1001 order) instead of taken as U+7A00 + n.
"""
import struct, sys, zlib

KS = [ord(bytes([0xB0 + i // 94, 0xA1 + i % 94]).decode('cp949')) for i in range(2350)]


def load(path):
    d = open(path, 'rb').read()
    head = struct.unpack_from('<4s9I', d, 0)
    labels, count, lsize, ssize, name = head[5:10]
    start = 41 + name if labels == 0 else len(d) - ssize
    print(f'{path}: {len(d)} bytes, version {head[1]:#x}, labels {labels}, strings {count}, '
          f'label bytes {lsize}, string bytes {ssize}, name {d[40:40 + name]!r}, strings at {start} '
          f'(41+name would be {41 + name}, end-size {len(d) - ssize})')
    return [r.decode('utf-8', 'replace') for r in d[start:].split(b'\0')[:count]]


MAP = {}


def font(path):
    a = open(path, 'rb').read()
    count = struct.unpack_from('<H', a, 6)[0]
    for i in range(count):
        e = 12 + i * 80
        name = a[e:e + 64].split(b'\0')[0].decode('ascii', 'replace')
        typ, csize, usize, off = struct.unpack_from('<4I', a, e + 64)
        if typ == 0x2d462600 and name.lower().endswith('font_festa'):
            g = zlib.decompress(a[off:off + csize])
            n, nm = struct.unpack_from('<I', g, 0x1c)[0], struct.unpack_from('<I', g, 0x40)[0]
            base = 0x44 + nm + 1
            if base + n * 20 != len(g):
                base = len(g) - n * 20
            codes = [struct.unpack_from('<I', g, base + i * 20)[0] for i in range(n)]
            first = next(i for i, c in enumerate(codes) if c >= 0x7A00)
            mine = codes[first:first + 2350]
            print(f'{name}: {n} glyphs; the 2350 from #{first}: codes {mine[0]:#x}..{mine[-1]:#x}, '
                  f'sorted {mine == sorted(mine)}, consecutive {mine == list(range(0x7A00, 0x7A00 + 2350))}')
            return {c: KS[i] for i, c in enumerate(mine)}
    raise SystemExit('no font_festa GFD in ' + path)


def ko(s):
    if MAP:
        return ''.join(chr(MAP[ord(c)]) if ord(c) in MAP else c for c in s)
    return ''.join(chr(KS[ord(c) - 0x7A00]) if 0x7A00 <= ord(c) < 0x7A00 + 2350 else c for c in s)


if len(sys.argv) > 5:
    MAP.update(font(sys.argv[5]))
xx, gu = load(sys.argv[1]), load(sys.argv[2])
first = int(sys.argv[3]) if len(sys.argv) > 3 else 0
n = int(sys.argv[4]) if len(sys.argv) > 4 else 16
for i in range(first, first + n):
    a = ko(xx[i])[:40] if i < len(xx) else '-'
    b = gu[i][:40] if i < len(gu) else '-'
    print(f'{i:5d}  {a!r:44}  {b!r}')

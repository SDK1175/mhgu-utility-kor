"""Korean fork, diagnosis: an MHXX GMD next to the MHGU one, string by string (prints to your terminal only).

    python3 tools/kor/peek.py <MHXX gmd> <MHGU _eng.gmd> [first index] [count]
"""
import struct, sys

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


def ko(s):
    return ''.join(chr(KS[ord(c) - 0x7A00]) if 0x7A00 <= ord(c) < 0x7A00 + 2350 else c for c in s)


xx, gu = load(sys.argv[1]), load(sys.argv[2])
first = int(sys.argv[3]) if len(sys.argv) > 3 else 0
n = int(sys.argv[4]) if len(sys.argv) > 4 else 16
for i in range(first, first + n):
    a = ko(xx[i])[:40] if i < len(xx) else '-'
    b = gu[i][:40] if i < len(gu) else '-'
    print(f'{i:5d}  {a!r:44}  {b!r}')

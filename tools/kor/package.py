"""Korean fork: the release zip, from the plugin build/hunters_compendium.nro (run tools/kor/build.sh first).

    python3 tools/kor/package.py 0.3-kor

Writes build/kor/MHGU-Utility-KO-<version>.zip. Holds no game text and nothing of the Korean patches: those are read
from the player's own files at run time.
"""
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
version = sys.argv[1] if len(sys.argv) > 1 else '0.3-kor'
nro = ROOT / 'build' / 'hunters_compendium.nro'
if not nro.is_file():
    sys.exit('no build/hunters_compendium.nro: run bash tools/kor/build.sh first')

README = f"""MHGU Utility 한글판 {version} (비공식)

원본: MHGU Utility (Hunter's Compendium) by Rikiar73574 (MIT License)
  https://github.com/Rikiar73574/mhgu-utility
한글판: https://github.com/SDK1175/mhgu-utility-kor
MHXX Recomp(덥크 PC 포팅) 0.10.8 이상에서 쓰도록 만든 한글판입니다.

[설치]
1. 압축 안의 hunters-compendium 폴더를 MHXX Recomp의 mods 폴더에 그대로 넣습니다.
   (원본 영어판 hunters-compendium이 있으면 지우고 넣으세요)
2. 게임을 켜고 F6을 누르면 열립니다.

[한글 표시]
- 메뉴·설명은 한글판에 들어 있습니다.
- 아이템·스킬·퀘스트 이름은 덥크 한글 패치가 깔려 있으면 게임 실행 중에 그 파일에서 읽어 옵니다.
  이 압축에는 한글 패치의 어떤 내용도 들어 있지 않습니다.
- 무기·방어구 이름: 덥크 한글 패치는 이 이름들을 일본어로 두었습니다.
  영미판(MHGU) 한글 패치를 가지고 있다면, 그 안의 romfs\\nativeNX\\eng\\table 폴더의 .gmd 파일들을
  hunters-compendium\\romfs\\nativeNX\\plugins\\hunters-compendium-ko\\table 폴더에 복사하세요.
  (없으면 무기·방어구 이름만 영어로 나옵니다)
  주의: 모드 폴더 안에 nativeNX\\eng 폴더를 만들면 게임이 켜지지 않습니다. 꼭 위 경로에 넣으세요.

[검색]
- 한/영 키 또는 오른쪽 Alt로 한글 입력을 켜고 끕니다. 초성 검색(예: ㄹㅇㅅ)도 됩니다.

[글꼴]
Paperlogy (PT&, 이주임), SIL Open Font License 1.1 - Paperlogy-OFL.txt
"""

# Nothing of the builder's PC in a file for others: user name, home or project folders (build paths in panics)
import os
data = nro.read_bytes()
marks = [b'\\Users\\', b'/Users/', b'/home/', b'C:/Users', str(ROOT).encode(), ROOT.as_posix().encode()]
for v in ('USERNAME', 'USER'):
    if os.environ.get(v):
        marks.append(os.environ[v].encode())
found = sorted({m.decode('utf-8', 'replace') for m in marks if len(m) > 2 and m in data})
if found:
    sys.exit(f'STOP: the plugin holds paths of this PC ({len(found)} kind(s)); not packaged. '
             'Pull the latest code, build again (bash tools/kor/build.sh), then package.')

out = ROOT / 'build' / 'kor' / f'MHGU-Utility-KO-{version}.zip'
out.parent.mkdir(parents=True, exist_ok=True)
base = 'hunters-compendium/romfs/nativeNX/plugins/'
with zipfile.ZipFile(out, 'w', zipfile.ZIP_DEFLATED) as z:
    z.write(nro, base + 'hunters_compendium.nro')
    z.writestr(base + 'hunters-compendium-ko/table/여기에_영미판_한글패치_table_gmd_넣기.txt',
               '영미판(MHGU) 한글 패치의 romfs\\nativeNX\\eng\\table\\*.gmd 를 이 폴더에 복사하세요. (선택)\n')
    z.writestr('읽어주세요.txt', README.replace('\n', '\r\n').encode('utf-8-sig'))
    for f in ['LICENSE', 'THIRD_PARTY_NOTICES.md', 'RUNTIME_NOTICES.md', 'MHGenDatabase-LICENSE.txt',
              'WEAPON-ICONS-LICENSE.txt']:
        if (ROOT / f).is_file():
            z.write(ROOT / f, 'licenses/' + f)
    for f in sorted((ROOT / 'licenses').iterdir()):
        z.write(f, 'licenses/' + f.name)
print(out, out.stat().st_size, 'bytes')

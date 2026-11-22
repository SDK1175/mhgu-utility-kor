# MHGU Utility 한글판 (MHXX Recomp용)

[Rikiar73574님의 MHGU Utility (Hunter's Compendium)](https://github.com/Rikiar73574/mhgu-utility)를 **[MHXX Recomp](https://github.com/SDK1175/MHXXRecomp)(몬스터헌터 더블크로스 일어판 PC 포팅) + 덥크 한글 패치**에서 한글로 쓰도록 고친 비공식 한글판입니다. 원작은 MIT 라이선스이며, 기능은 원작 그대로입니다.

- 영미판(MHGU)이나 MHGU Recomp에서 영어로 쓰실 분은 [원작](https://github.com/Rikiar73574/mhgu-utility/releases)을 받으세요. 원작 설명은 [docs/README.upstream.md](docs/README.upstream.md)에 그대로 있습니다.
- 이 한글판의 문제는 원작자가 아니라 [이 저장소의 이슈](https://github.com/SDK1175/mhgu-utility-kor/issues)나 [MHXX Recomp 이슈](https://github.com/SDK1175/MHXXRecomp/issues)로 알려 주세요.

[받기 (릴리즈)](https://github.com/SDK1175/mhgu-utility-kor/releases/latest)

## 무엇을 하나

게임을 하면서 아이템, 무기 강화 트리, 방어구, 생산 소재, 채집·몬스터 입수처를 찾아보고, 위시리스트로 모을 소재를 관리합니다.

- **아이템:** 분류별 검색, 설명, 맵·몬스터·퀘스트별 입수처(랭크, 수량, 확률, 부위파괴·포획 조건)
- **무기 트리:** 무기 종류별 강화 트리, 성능, 예리도, 생산 소재
- **방어구:** 세트·부위별, 랭크·헌터 타입·성별 필터, 방어력·내성·슬롯·스킬·소재
- **위시리스트:** 무기·방어구를 담아 두면 아이템 상자와 포치의 **실제 보유 수량**으로 모자란 소재를 보여 주고, 퀘스트 중에는 작은 추적 창으로 표시
- **퀘스트 추천:** 위시리스트 소재를 모을 수 있는, 열려 있는 퀘스트를 추천
- **스킬 조합(Set Builder):** 원하는 스킬로 방어구 조합 검색. 캐릭터의 성별·헌터 타입·무기 슬롯·HR을 게임에서 읽고, 가진 호석을 모두 확인
- **채집 지도:** 퀘스트 중 고른 아이템이 나오는 채집 포인트를 지도에 표시 (Map Assist와 같이 쓸 수 있음)

## 한글판에서 바뀐 것

- 메뉴·라벨·안내 문구, 스킬 설명 한글 번역
- **아이템·스킬·퀘스트 이름:** 깔려 있는 **덥크 한글 패치**에서 게임 실행 중에 읽어 옵니다. 이 플러그인에는 한글 패치 내용이 들어 있지 않습니다.
- **무기·방어구 이름:** 덥크 한글 패치는 이 이름들을 일본어로 두었습니다. 영미판(MHGU) 한글 패치를 가지고 있다면 그 table 파일을 넣어 한글로 볼 수 있습니다 (아래 "설치" 3번). 없으면 무기·방어구 이름만 영어로 나옵니다.
- **한글 검색:** 한/영 키 또는 오른쪽 Alt로 한글 입력, 초성 검색(예: ㄹㅇㅅ), 가나다 정렬
- **덥크에 맞춘 부분:** 캐릭터·인벤토리·채집 지도 등 게임 데이터를 덥크 위치에서 읽음, 빠르게 쳐도 순서대로 들어가는 검색 입력, F1에서 바꾼 패드 버튼 배치를 따르는 버튼 표시

## 필요한 것

- **MHXX Recomp 0.10.8 이상** (플러그인 마우스·키보드 입력, 영문판 주소 변환이 들어 있습니다)
- 덥크 한글 패치 (게임 이름을 한글로 보려면)
- 선택: 영미판(MHGU) 한글 패치의 table 파일 (무기·방어구 이름)

## 설치

1. 게임을 종료합니다.
2. `MHGU-Utility-KO-<버전>-kor.zip`의 압축을 풀어, 안의 `hunters-compendium` 폴더를 MHXX Recomp의 `mods` 폴더에 넣습니다. 원작 영어판이 있으면 지우고 넣으세요.
3. (선택) 영미판 한글 패치를 가지고 있다면, 그 안의 `romfs\nativeNX\eng\table\` 폴더의 `.gmd` 파일들을
   `mods\hunters-compendium\romfs\nativeNX\plugins\hunters-compendium-ko\table\`에 복사합니다.
   **주의:** 모드 폴더 안에 `nativeNX\eng` 폴더를 만들면 게임이 켜지지 않습니다. 꼭 위 경로에 넣으세요.
4. 게임을 켭니다.

```text
mods/
└── hunters-compendium/
    └── romfs/
        └── nativeNX/
            └── plugins/
                ├── hunters_compendium.nro
                └── hunters-compendium-ko/
                    └── table/            (선택: 영미판 한글 패치의 table .gmd)
```

## 사용 방법

- **열기/닫기:** 키보드 **F6**, 패드 **Start + 십자키 왼쪽**
- **검색:** 키보드로 바로 치면 됩니다. 한/영 키(또는 오른쪽 Alt)로 한글 입력을 켜고 끕니다.
- **스킬 조합:** 마을에서 캐릭터를 불러온 뒤 쓰세요. 스킬을 고르고 검색하면 조합이 나옵니다.
- **위시리스트 → 퀘스트 추천:** 위시리스트에서 Y(키보드 F9)
- **채집 지도:** 퀘스트 중에 열어서 아이템을 고르면, 창을 닫은 뒤에도 지도에 그 채집 포인트가 표시됩니다.
- 퀘스트 중 시작 메뉴의 "MHGU Utility" 항목은 덥크에서 아직 확인 중입니다. F6이나 Start + 왼쪽을 쓰세요.
- 화면 크기, 오트모 퀘스트 포함 여부: Insert → Forge 설정 → `Plugin UI` → `MHGU Utility`

## 저장 데이터

설정(`compendium_settings.ini`)과 위시리스트(`compendium_wishlist.txt`)는 게임의 세이브 폴더에 따로 저장됩니다. MHXX Recomp의 기본 경로는 `%APPDATA%\MHXXRecomp\save\save_0\`입니다. 플러그인을 업데이트해도 지워지지 않습니다.

## 버전

원작 버전 뒤에 `-kor`를 붙입니다 (예: 원작 0.3 → `0.3-kor`).

## 빌드 (개발자용)

MSYS2(UCRT64), Rust nightly, Python 3가 필요합니다. 생성 데이터(아이템·무기 표 등)는 저장소에 들어 있지 않아서, 빌드할 때 **본인이 가진 MHGU 1.4.0 게임 파일**을 입력으로 씁니다 (원작과 같은 방식, [docs/BUILD.md](docs/BUILD.md)). 경로는 `tools/kor/build.sh` 위쪽에서 바꿉니다.

```bash
bash tools/kor/setup.sh <MHGU 1.4.0 romfs가 있는 폴더>   # 처음 한 번 (느림)
bash tools/kor/build.sh                                  # 빌드하고 MHXX Recomp mods 폴더에 넣기
python3 tools/kor/package.py 0.3-kor                     # 배포용 zip (build/kor/)
```

`package.py`는 플러그인 안에 PC 사용자 이름이나 폴더 경로가 남아 있으면 zip을 만들지 않고 멈춥니다.

## 라이선스

MIT License (원작: Copyright (c) 2026 Riccardo Testa). `LICENSE`, `THIRD_PARTY_NOTICES.md`, `RUNTIME_NOTICES.md`, `licenses/`를 참고하세요.
한글 글꼴: [Paperlogy](https://github.com/Freesentation/paperlogy) (PT&, 이주임), SIL Open Font License 1.1.

Monster Hunter는 CAPCOM의 상표이며, 이 프로젝트는 CAPCOM과 관계없습니다.

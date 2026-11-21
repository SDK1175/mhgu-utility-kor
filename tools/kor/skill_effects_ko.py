"""Korean fork: tools/skill_effects.py (the author's reviewed skill mechanics) translated.

Same keys and structure as the English module; tools/set_data.py takes this one when it exists. Names of skills and
items inside a text are written {{English name}}: the plugin puts the game's own Korean name there when it draws the
text (mod/src/ko.rs, expand), so they read as the Korean patch names them.
"""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import skill_effects as en  # noqa: E402  (SOURCE_OVERRIDES, GUIDE, pack: unchanged)

GUIDE, JP, SOURCE_OVERRIDES, pack = en.GUIDE, en.JP, en.SOURCE_OVERRIDES, en.pack

BASE = {
 'No Skill': '발동 중인 스킬 효과가 없습니다.',
 'Negate Poison': '독과 맹독을 막습니다. 극독은 일반 독으로 약해질 뿐, 완전히 막지는 못합니다.',
 'Double Poison': '독 상태에서 체력이 깎이는 피해 x2.',
 'Negate Paralysis': '헌터가 마비되지 않습니다. 장착 중 항상 적용.',
 'Double Paralysis': '마비될 때마다 마비 시간 x2.',
 'Negate Sleep': '수면 상태가 되지 않습니다. 장착 중 항상 적용.',
 'Double Sleep': '잠들 때마다 수면 시간 x2.',
 'Negate Stun': '기절을 완전히 막습니다. 장착 중 항상 적용.',
 'Halve Stun': '기절할 때 50% 확률로 기절을 막습니다. 기절 시간이 반으로 줄거나 축적치가 두 배가 되는 것은 아닙니다.',
 'Double Stun': '기절에서 풀리는 데 필요한 입력 횟수 x2.',
 'HG Earplugs': '작은 포효와 큰 포효에 움찔하지 않습니다. 포효에 실린 피해나 다른 효과는 막지 못합니다.',
 'Earplugs': '작은 포효에 움찔하지 않습니다. 큰 포효에는 여전히 멈춥니다.',
 'Windproof (Hi)': '약한 풍압과 강한 풍압을 막습니다. 용풍압은 강한 풍압으로 줄어들 뿐, 완전히 막지는 못합니다.',
 'Windproof (Lo)': '약한 풍압에 휘청이지 않습니다. 강한 풍압과 용풍압은 그대로 받습니다.',
 'Tremor Res': '몬스터 공격의 진동에 휘청이지 않습니다. 공격의 피해는 그대로 받습니다.',
 'Negate Bind': '눈사람, 뼈 상태, 심한 거품 상태의 움직임 제한을 막습니다. 점액은 막지 못합니다.',
 'Heat Cancel': '더운 지역의 체력 감소와 용암 지대 피해를 막습니다. {{Cool Drink}}가 필요 없습니다. 불 속성 상태이상은 막지 못합니다.',
 'Heat Surge': '더운 지역과 용암 지대에서 체력이 깎이는 피해 x2.',
 'Cold Cancel': '추운 지역에서 최대 스태미나가 줄지 않습니다. {{Hot Drink}}가 필요 없습니다.',
 'Cold Surge': '추위로 최대 스태미나가 두 배 빨리 줄어듭니다.',
 'Polar Hunter': '추운 지역에서 물리 공격력 +15, 방어력 +20, 추위 무시. {{Cool Drink}} 효과 중에는 어느 지역에서든 물리 공격력 +5.',
 'Tropic Hunter': '더운 지역에서 물리 공격력 +15, 방어력 +20, 더위·용암 피해 무시. {{Hot Drink}} 효과 중에는 어느 지역에서든 방어력 +10.',
 'Anti-Theft': '몬스터가 파우치의 아이템을 훔쳐 가지 못합니다.',
 'Iron Skin': '방어력 저하와 심한 방어력 저하를 모두 막습니다.',
 'Antivirus': '광룡 바이러스를 극복하면 회심률 보너스가 +15%에서 +30%로 오릅니다. 극복도 쉬워집니다. 감염·극복 전에는 효과가 없습니다.',
 'Bio Researcher': '악취와 폭파 상태이상을 막고, {{Dung Bomb}} 조합이 100% 성공합니다. 거름탄에 약한 몬스터는 맞으면 다른 지역으로 이동하려 합니다(이동할 수 있을 때).',
 'Bio Master': '{{Bio Researcher}} 효과(악취·폭파 상태이상 무효, 거름탄 조합 100%, 도주 유도)에 더해 광룡 바이러스 감염을 늦춥니다. 감염률 보정의 정확한 값은 미검증입니다.',
 'Negate Bleeding': '출혈 상태가 되지 않습니다. 장착 중 항상 적용.',
 'Double Bleeding': '출혈로 깎이는 체력 x2.',
 'Blightproof': '불·물·번개·얼음·용 속성 상태이상(심한 상태 포함)을 막습니다. 폭파 상태이상은 막지 못합니다.',
 'Element Atk Up': '무기 속성치 x1.10. 속성별 공격 스킬과 합친 배율은 x1.20이 상한이며, 고정 가산은 따로 적용됩니다.',
 'Element Atk Down': '무기 속성치 x0.90. 무기에 원래 있는 속성에만 적용됩니다.',
 'Status Atk +1': '독·마비·수면 축적치 x1.10 후 +1. 독의 지속 피해나 폭파에는 적용되지 않습니다.',
 'Status Atk +2': '독·마비·수면 축적치 x1.20 후 +1. 독의 지속 피해나 폭파에는 적용되지 않습니다.',
 'Status Atk Down': '독·마비·수면 축적치 x0.90. 폭파에는 적용되지 않습니다.',
 'Speed Sharpening': '숫돌질이 4번 대신 1번에 끝납니다(75% 감소). 회복되는 예리도 양은 같습니다.',
 'Slow Sharpening': '숫돌질이 4번 대신 5번 걸립니다.',
 'Sharpness +1': '예리도를 최대 25칸 늘립니다. 무기의 늘어나는 예리도 구간에만 적용되며, 늘어나지 않는 무기에는 효과가 없습니다.',
 'Sharpness +2': '예리도를 최대 50칸 늘립니다. 무기의 늘어나는 예리도 구간에만 적용되며, 늘어나지 않는 무기에는 효과가 없습니다.',
 'Razor Sharp': '예리도 1 소모는 50% 확률로 0이 됩니다. 2 이상 소모는 절반(내림)이 됩니다.',
 'Blunt Edge': '예리도를 소모하는 공격의 소모량 x2.',
 "Mind's Eye": '예리도·육질 때문에 튕기지 않습니다. 피해가 늘거나, 튕길 공격의 추가 예리도 소모가 없어지지는 않습니다.',
 'Blind Eye': '예리도·육질상 튕기지 않을 공격도 33% 확률로 튕깁니다.',
 'Heavy Polish': '숫돌질 후 60초 동안 근접 무기의 예리도가 줄지 않고, 거너의 치명 거리 배율이 x1.50 대신 x1.75가 됩니다. {{Item Use Up}}/{{Item Use Down}}이 시간을 바꿉니다.',
 'Bludgeoner': '현재 예리도에 따라 물리 공격력 배율: 초록 x1.10, 노랑 x1.15, 주황·빨강 x1.20. 파랑 이상은 근접 보너스가 없습니다.',
 'Critical Draw': '발도 공격의 회심률 +100%p(최종 100% 상한). 수렵기술은 발도 공격으로 치지 않습니다.',
 'Punishing Draw': '물리 공격력 +5. 절단 발도 공격은 기절치 30, 감기치 20도 줍니다. 쌍검은 타마다 값이 다르며 미검증입니다.',
 'Quick Sheath': '납도 동작 속도 x1.30(동작 시간 약 23% 감소). 실제 전체 시간은 무기·스타일에 따라 다릅니다. 브레이브 납도 캔슬 준비도 빨라집니다.',
 'Challenge Sheath': '같은 지역에 분노한 대형 몬스터가 있을 때 납도하면 35% 확률로 예리도 회복: 대검 4, 태도 10, 한손검·쌍검·랜스 15, 해머·수렵피리 11, 건랜스·차지액스 16, 슬래시액스 14, 조충곤 12. 태도 자동 납도에는 발동하지 않습니다.',
 'Bladescale Hone': '발도 중 구르기 5번 또는 스텝 9번마다 예리도 5 회복(세르레기오스 근접 무기는 7). 보우건: 회피할 때마다 50% 확률로 선택한 탄 1발 장전, 세르레기오스면 1발 확정 + 50% 확률로 1발 더. 활 접격병: 물리 x1.50(세르레기오스 x1.65).',
 'Normal/Rapid Up': '통상탄과 연사 화살의 물리 피해 x1.10. 속성 피해는 늘지 않습니다.',
 'Pierce/Pierce Up': '관통탄과 관통 화살의 물리 피해 x1.10. 속성 피해는 늘지 않습니다.',
 'Pellet/Spread Up': '산탄 물리 피해 x1.20, 확산 화살 물리 피해 x1.30.',
 'Heavy/Heavy Up': '강사탄과 강사 화살 피해 x1.10.',
 'Bonus Shot': '라이트보우건 속사 1회에 1발을 더 쏩니다. 한 발의 피해는 늘지 않습니다.',
 'Shot Booster': '치명 거리 범위를 50% 늘립니다. 치명 거리 피해 배율은 그대로입니다.',
 'Load Up': '보우건·건랜스: 장전 수 +1. 활: 원래 잠긴 모으기 4단계를 엽니다. 차지액스: 노란 차지 병 3개 대신 5개, 빨간 차지 5개 대신 최대 10개.',
 'TrueShot Up': '보우건 내장탄 피해 x1.20. 활 강격·강사: 물리 x1.20, 곡사: 물리 x1.40. 활의 속성 피해는 그대로입니다.',
 'Ammo Saver': '사격·병 사용 때마다 20% 확률로 소모하지 않습니다. 보우건 탄은 자동 장전되지 않고 소지 탄으로 돌아갑니다.',
 'Weakness Exploit': '맞힌 부위의 해당 공격 종류 육질이 45 이상이면 회심률 +50%p.',
 'Repeat Offender': '1타 후 회심률 +25%, 5연속 타격 후 합계 +30%(+55% 아님). 맞힐 때마다 5초 타이머가 갱신되고, 5초 동안 못 맞히면 사라집니다.',
 'Status Crit': '회심 공격 때 독·마비·수면 축적치 x1.20. 폭파나 독 지속 피해는 늘지 않습니다.',
 'Elemental Crit': '회심 공격 때 속성 배율: 한손검·쌍검·활 x1.35, 라이트·헤비보우건 x1.30, 대검 x1.20, 그 밖의 무기 x1.25.',
 'Critical Boost': '회심 공격의 물리 피해가 x1.25 대신 x1.40. 회심률이 오르거나 마이너스 회심이 바뀌지는 않습니다.',
 'Bitter Affinity': '마이너스 회심이 나면 25% 확률로 x0.75 대신 x2.00 물리 배율이 됩니다. {{Critical Boost}}·{{Elemental Crit}}·{{Status Crit}}이 적용되는 일반 회심은 아닙니다.',
 'Focus': '대검·해머·활 모으기 시간 x0.80. 태도 기인 게이지, 쌍검 귀인 게이지, 슬래시액스·차지액스 게이지 상승 x1.25. 라이트보우건 수렵기술 게이지도 빨라지며, 정확한 배율은 미검증입니다.',
 'Distraction': '대검·해머·활 모으기 시간 x1.20. 태도 기인 게이지, 쌍검 귀인 게이지, 슬래시액스·차지액스 게이지 상승 x0.75. 라이트보우건 수렵기술 게이지도 느려집니다.',
 'Marathon Runner': '계속 줄어드는 스태미나(달리기·귀인화·모으기) 소모 x0.50. 회피·가드의 고정 소모는 그대로입니다.',
 'Short Sprinter': '계속 줄어드는 스태미나 소모 x1.20(+20%). 회피·가드의 고정 소모와 이동 속도는 그대로입니다.',
 'Constitution +1': '회피·가드 같은 고정 스태미나 소모 x0.75(25% 감소). 달리기의 지속 소모는 그대로입니다.',
 'Constitution +2': '회피·가드 같은 고정 스태미나 소모 x0.50(50% 감소). 달리기의 지속 소모는 그대로입니다.',
 'Constitution -1': '회피·가드 같은 고정 스태미나 소모 x1.20.',
 'Constitution -2': '회피·가드 같은 고정 스태미나 소모 x1.35.',
 'Stam Recov Up': '스태미나가 회복될 때 회복 속도 x2.00.',
 'Stam Recov Down': '스태미나가 회복될 때 회복 속도 x0.50.',
 'Endurance Runner': '스태미나 25 이하에서 달리면 달리기의 지속 소모가 멈춥니다. 고정 소모는 여전히 스태미나를 씁니다.',
 'Evasion +1': '일반 회피의 무적 시간이 6프레임 대신 10프레임(30fps 기준, 0.200초 대신 0.333초). 작은 거품의 같은 회피 효과와는 겹치지 않습니다.',
 'Evasion +2': '일반 회피의 무적 시간이 6프레임 대신 12프레임(30fps 기준, 0.200초 대신 0.400초).',
 'Evasion Down': '일반 회피의 무적 시간이 6프레임 대신 3프레임(30fps 기준, 0.200초 대신 0.100초).',
 'Evade Extender': '일반 회피의 이동 거리 x1.50. 무적 시간이나 수렵기술 회피는 늘지 않습니다.',
 'Bubbly Dance': '심한 거품 상태를 막습니다. 구르기 3번 또는 스텝 6번 후 작은 거품이 생겨 {{Evasion +1}} 효과와 고정 스태미나 소모 25% 감소를 줍니다. 스태미나 감소는 {{Constitution +1}} 계열과 겹치며, 맞거나 경직되면 거품이 터집니다.',
 'Guard +1': '가드 성능 +10. 가드 단계가 바뀌면 밀려남·깎이는 피해·스태미나 소모가 줄어듭니다. 고정 피해 감소는 아닙니다.',
 'Guard +2': '가드 성능 +20. 가드 단계가 바뀌면 밀려남·깎이는 피해·스태미나 소모가 줄어듭니다. 고정 피해 감소는 아닙니다.',
 'Guard -1': '가드 성능 -10. 같은 공격을 막아도 더 큰 밀려남·피해·스태미나 소모 단계가 될 수 있습니다.',
 'Guard Up': '가드 강화가 있어야만 막을 수 있는 공격(빔·가스 다수 포함)을 막을 수 있습니다. 잡기나 원래 못 막는 모든 공격을 막지는 않습니다.',
 'Knockout King': '이미 기절치가 있는 공격의 기절치 x1.10. 일반 절단 공격에 기절치가 생기지는 않습니다. 엽충의 기절치는 제외됩니다.',
 'Stamina Thief': '이미 감기치가 있는 공격의 감기치 x1.20. 체력 피해나 기절치는 늘지 않습니다.',
 'Horn Maestro': '수렵피리 선율별 효과 시간을 늘립니다. 예: 공격력 강화(대) 90초 -> 120초. 피리·호루라기 파손과 회복 선율 약화도 줄이며, 정확한 확률은 미검증입니다.',
 'Artillery Novice': '대포·발리스타 x1.10, 철갑유탄·삼연탄·용격탄·대포탄 물리 x1.15, 차지액스 임팩트 병 폭발 x1.30, 건랜스 포격 고정 피해 x1.10. 건랜스 히트 게이지 변화는 3분의 1 줄어듭니다.',
 'Artillery Expert': '대포·발리스타 x1.20, 철갑유탄·삼연탄·용격탄·대포탄 물리 x1.30, 차지액스 임팩트 병 폭발 x1.35, 건랜스 포격 고정 피해 x1.20. 건랜스 히트 게이지 변화 절반, 용격포 재사용 120초 -> 90초.',
 'Partbreaker': '아직 부서지지 않은 부위의 파괴·절단 축적치 x1.30. 체력 피해는 늘지 않으며, 부위마다 공격 종류·파괴 조건은 그대로입니다.',
 'Bombardier': '폭탄 아이템 피해 x1.30, 폭파 축적치 x1.20, 폭탄 조합 성공률 100%. 폭파 폭발의 피해는 늘지 않습니다.',
 'Latent Power +1': '90초 동안 회심률 +30%, 스태미나 소모 50% 감소. 피해 180 누적 또는 대형 몬스터와 300초 전투 후 발동합니다.',
 'Latent Power +2': '90초 동안 회심률 +50%, 스태미나 소모 75% 감소. 피해 180 누적 또는 대형 몬스터와 300초 전투 후 발동합니다.',
 'Challenger +1': '같은 지역의 대형 몬스터가 분노한 동안 물리 공격력 +10, 회심률 +10%.',
 'Challenger +2': '같은 지역의 대형 몬스터가 분노한 동안 물리 공격력 +20, 회심률 +15%.',
 'Peak Performance': '현재 체력이 최대 체력과 같을 때만 물리 공격력 +20. 빨간 회복 가능 체력은 가득 찬 것으로 치지 않습니다.',
 'Trump Card': '대형 몬스터가 다운·구속되면 60초 버프: 피해를 주는 수렵기술은 대체로 물리 x1.20, 시간형 수렵기술은 지속 시간 x1.20. 수렵기술별 예외는 전체 효과에 있습니다.',
 'Dragonheart': '체력이 최대의 2/3 이하일 때 물리 공격력 x1.10, 모든 속성 내성 50, 특수한 용 속성 상태로 용 이외의 무기 속성·상태이상이 봉인됩니다. 일반 용 속성 상태이상도 막습니다.',
 'Adrenaline +1': '체력 40% 이하에서 원래의 저체력 방어 보너스에 방어력 +15를 더합니다. 공격력은 오르지 않습니다.',
 'Adrenaline +2': '체력 40% 이하에서 물리 공격력 x1.30, 원래의 저체력 방어 보너스에 방어력 +15.',
 'Worrywart': '체력 40% 이하에서 물리 공격력 x0.70, 원래의 저체력 방어 보너스가 9 줄어듭니다.',
 'Fortify': '자신이 수레에 실릴 때마다 공격력 +10%, 방어력 +15%. 최대 2단계(공격력 +20%, 방어력 +30%). 동료가 수레에 실려도 발동하지 않습니다.',
 'Resentment': '체력 바에 빨간 회복 가능 체력이 남아 있는 동안 물리 공격력 +20.',
 'Resuscitate': '해당하는 상태이상이나 속성 상태이상에 걸린 동안 물리 공격력 +20. {{Bubbly Dance}}의 작은 거품 상태도 해당됩니다.',
 'Guts': '공격 전 체력이 65 이상이면 목숨당 한 번, 죽을 공격을 체력 1로 버팁니다. 수레 후 초기화되며, 지속 피해나 두 번째 공격에는 쓰러질 수 있습니다.',
 'Sneak': '행동으로 쌓이는 어그로를 낮춰 몬스터가 덜 노리게 합니다. 노림은 다른 헌터·행동·몬스터 AI에도 달려 있으며, 일정한 확률 수치는 미검증입니다.',
 'Taunt': '행동으로 쌓이는 어그로를 높입니다. 반드시 노려지는 것은 아니며, 정확한 어그로 양과 확률은 미검증입니다.',
 'Palico Rally': '동행 오토모 아이루의 공격력·방어력 x1.10(내림). MHGU에는 예전의 샤카라카 스태미나 회복 효과가 없습니다.',
 'Palico Cheer': '주먹 쥐기 제스처로 쓰러져 회복 중인 오토모를 불러옵니다. 재사용 대기 600초.',
 'Mounting Master': '탑승 축적치와 탑승 다운 게이지 상승 x1.25. 일반 공중 공격의 피해는 늘지 않습니다.',
 'Saddle Sore': '탑승 축적치와 탑승 다운 게이지 상승 x0.90.',
 'Airborne': '해당 점프 공격의 물리 피해, 탑승 축적치, 공중 다운 축적치 x1.10. 탑승 배율은 합쳐서 x1.40이 상한입니다. 지상 공격은 그대로입니다.',
 'Composed': '수렵기술·브레이브·연금 게이지 상승 x1.15.',
 'Chance Seizer': '몬스터에게 피해를 받으면 수렵기술·브레이브·연금 게이지가 찹니다. 피해량에 따라 다르며, 정확한 환산 계수는 미검증입니다.',
 'Longer SP': 'SP 상태 지속 시간 x1.50. 수렵기술 동작 자체가 아니라 SP 보조 효과의 시간에 적용됩니다.',
 'Autotracker': '페인트볼 없이도 대형 몬스터를 항상 지도에 표시하고, 종류·방향·경계 상태도 보여줍니다.',
 'Detect': '표시된 대형 몬스터의 종류·방향·경계 색(파랑 미발견, 노랑 경계, 빨강 전투)을 보여줍니다. 페인트볼 같은 표시 수단은 여전히 필요합니다.',
 'Capture Guru': '표시된 몬스터의 체력이 그 종의 포획 가능 기준에 닿으면 지도 아이콘이 하얗게 깜빡입니다. 기준이 바뀌거나 함정이 반드시 성공하지는 않습니다.',
 'Outdoorsman': '{{Map}}을 가지고 있지 않아도 지도가 항상 보입니다. 낚시 입질과 고기 굽기의 타이밍 범위가 50% 늘어납니다.',
 'Pro Transporter': '무거운 운반 아이템을 가벼운 아이템 속도(이동 속도 약 +33%)로 나릅니다. 가벼운 아이템은 빨라지지 않습니다. 더 높은 곳에서도 떨어질 수 있으며, 안전한 높이는 미검증입니다.',
 'Divine Blessing': '피해를 받을 때 25% 확률로 피해 30% 감소(x0.70).',
 'Demonic Blessing': '피해를 받을 때 25% 확률로 피해 30% 증가(x1.30).',
 "Hero's Talisman": '방어력 적용 후 피해가 5 미만인 공격의 피해와 경직·상태이상을 없앱니다.',
 'Recovery Up': '해당 회복 효과의 회복량 x1.25. 빨간 체력의 자연 회복은 빨라지지 않습니다.',
 'Recovery Down': '해당 회복 효과의 회복량 x0.75.',
 'Recovery Spd +1': '빨간 체력의 자연 회복 x2.00. 일반 초록 체력 회복은 그대로입니다.',
 'Recovery Spd +2': '빨간 체력의 자연 회복 x4.00. 일반 초록 체력 회복은 그대로입니다.',
 'Recovery Spd -1': '빨간 체력의 자연 회복 x0.50.',
 'Recovery Spd -2': '빨간 체력의 자연 회복 x0.25.',
 'Item Use Up': '해당 소비 아이템 버프의 지속 시간 x1.50. 예: {{Heavy Polish}} 60초 -> 90초. 모든 시간제 아이템·수렵기술에 적용되지는 않습니다.',
 'Item Use Down': '해당 소비 아이템 버프의 지속 시간 x2/3. 예: {{Heavy Polish}} 60초 -> 40초.',
 'Wide-Range +1': '해당 아이템을 쓰면 같은 지역 헌터에게도 50% 효과. 해독·종자류는 아이템마다 다르게 처리됩니다. {{Potion}}, {{Herb}}, {{Antidote}}, {{Nulberry}}, {{Might Seed}}/{{Adamant Seed}} 포함.',
 'Wide-Range +2': '해당 아이템을 쓰면 같은 지역 헌터에게도 100% 효과. {{Potion}}, {{Herb}}, {{Antidote}}, {{Nulberry}}, {{Might Seed}}/{{Adamant Seed}} 포함. {{Mega Potion}}이나 모든 소비 아이템이 해당되지는 않습니다.',
 'Negate Hunger': '시간이 지나도 최대 스태미나가 줄지 않습니다. 행동으로 쓰는 스태미나는 그대로입니다.',
 'Halve Hunger': '시간에 따른 최대 스태미나 감소 x0.50. 25씩 줄어드는 간격이 6분에서 12분으로 늘어납니다.',
 'Raise Hunger': '25씩 줄어드는 최대 스태미나 감소 간격이 6분에서 4.5분으로 짧아집니다.',
 'Double Hunger': '25씩 줄어드는 최대 스태미나 감소 간격이 6분에서 3분으로 짧아집니다.',
 'Scavenger': '고기는 {{Gourmand}}의 최대 스태미나 +25를 받습니다. 그 밖의 해당 음식·음료는 1/3 확률로 최대 스태미나 25 회복(최대 150).',
 'Gourmand': '해당 고기가 스태미나를 회복하면 최대 스태미나를 25 더 회복합니다(확정). {{Well-done Steak}}는 50 대신 75. 최대 스태미나는 150이 상한입니다.',
 'Speed Eating +1': '고기만 빠른 먹기 동작을 씁니다. {{Potion}} 등 고기가 아닌 아이템은 그대로이며, 전체 동작 단축률은 미검증입니다.',
 'Speed Eating +2': '고기와 {{Potion}}을 포함한 먹고 마시는 아이템에 빠른 동작을 씁니다. 마지막 근육 포즈는 남으며, 총 시간은 아이템·동작에 따라 다릅니다.',
 'Slow Eater': '고기를 느린 동작으로 먹습니다. 다른 아이템은 그대로이며, 아이템별 정확한 시간은 미검증입니다.',
 'Rationer': '해당 아이템을 먹거나 마실 때 25% 확률로 소비하지 않습니다.',
 'Meat Lover': '{{Raw Meat}}를 먹을 수 있게 되어 최대 스태미나 50 회복. {{Burnt Meat}} 이외의 구운 고기는 먹기 시작부터 60초 동안 스태미나 무한(동작 후 약 55초).',
 'Mushromancer': '{{Blue Mushroom}}은 {{Potion}}, {{Dragon Toadstool}}은 {{Max Potion}}, {{Nitroshroom}}은 {{Demondrug}}, {{Mopeshroom}}은 {{Dash Juice}} 효과로 먹습니다. 다른 버섯은 전체 효과에 있습니다.',
 'Herbology': '{{Herb}}의 회복량이 30이 됩니다(다른 회복 보정 전).',
 'Pro Herbology': '{{Herb}}의 회복량이 50이 됩니다(다른 회복 보정 전).',
 'Combination Pro': '결과 수량이 바뀌는 조합에 성공하면 항상 최대 수량이 나옵니다. 조합 성공률은 오르지 않습니다.',
 'Trap Master': '함정 조합 100% 성공, 함정·폭탄 설치 속도 x2.00.',
 'Gathering +1': '채집 계속 확률이 25/32에서 27/32로 오릅니다(78.125% -> 84.375%). 나오는 아이템 종류는 그대로입니다.',
 'Gathering +2': '채집 계속 확률이 25/32에서 29/32로 오릅니다(78.125% -> 90.625%). 나오는 아이템 종류는 그대로입니다.',
 'Gathering -1': '채집 계속 확률이 25/32에서 21/32로 떨어집니다(78.125% -> 65.625%).',
 'Honey Hunter': '{{Honey}}나 {{Royal Honey}}를 채집할 때마다 1개 대신 2개.',
 'Charm Collector': '호석을 채굴할 때 20% 확률로 호석 하나를 더 얻습니다.',
 'Charm Chaser': '호석을 채굴할 때 30% 확률로 호석 하나를 더 얻습니다.',
 'Carving Pro': '갈무리 중 작은 넉백에 밀리지 않습니다. 갈무리 횟수가 늘거나 모든 피해 공격을 막지는 않습니다.',
 'Carving Celebrity': '해당 몸통·꼬리 갈무리 +1회, 갈무리 중 작은 넉백 방지. 갈무리 보상 확률은 그대로입니다.',
 'Capture Expert': '포획 보수가 2~3개 대신 최소 3개. 개별 아이템 확률은 오르지 않습니다.',
 'Capture Master': '포획 보수 최소 3개, 2/3 확률로 4개째 보수. 개별 아이템 확률은 오르지 않습니다.',
 'Torso Up': '동체 연장 부위 하나마다 몸통 방어구의 스킬 포인트(몸통 장식주 포함)를 한 번씩 복사합니다. N개면 몸통 포인트 x(N+1). 방어력·내성은 복사하지 않습니다.',
 'Skill +2': '합계가 0보다 큰 다른 모든 스킬 계통에 +2 포인트. 없는 계통이나 자기 자신에는 더하지 않습니다.',
 'Double Talisman': '호석의 스킬 포인트 x2(호석에 끼운 장식주 포함). 방어구·무기의 장식주는 두 배가 되지 않습니다.',
}

FULL = {
 'Mushromancer': '먹을 수 있는 버섯과 효과: {{Blue Mushroom}} -> {{Potion}}, {{Toadstool}} -> {{Nutrients}}, {{Dragon Toadstool}} -> {{Max Potion}}, {{Parashroom}}/{{Bindshroom}} -> {{Armorskin}}, {{Nitroshroom}} -> {{Demondrug}}, {{Super Mushroom}} -> {{Mega Potion}}, {{Mopeshroom}} -> {{Dash Juice}}, {{Choice Mushroom}} -> {{Mega Dash Juice}}, {{Abyssal Mushroom}} -> {{Nulberry}}, {{Reststool}} -> {{Antidote}}, {{Chaos Mushroom}} -> {{Psychoserum}}, {{Unique Mushroom}} -> {{Ration}}, {{Ripened Mushroom}} -> {{Ancient Potion}}. {{Great Shroom}}과 다른 희귀 이벤트 버섯의 효과는 미검증입니다.',
 'Trump Card': '발동: 대형 몬스터가 다운·구속되면 60초 버프. 피해를 주는 수렵기술 대부분은 물리 x1.20, 시간형 수렵기술 대부분은 지속 시간 x1.20. 예외: 절대회피·절대회피【장전】은 회피 2번, 광화주입은 회심 효과 즉시 부여, 사자분신의 다음 타격 배율 +15%p, 선율 계열은 2단계 선율, 용의 숨결 불·고정 피해 +20%, 귀인난무 효과 +15%, 풀하우스는 빠른 동작, 폭약 주입·전탄 장전은 5발 추가, 귀인연참 피해 +10%·지속 +20%. 부활의 불사조는 회복량이 늘지만 정확한 양은 미검증입니다.',
}

EL = {'Fire': '불', 'Water': '물', 'Thunder': '번개', 'Ice': '얼음', 'Dragon': '용'}
KIND = {'Normal': '통상탄', 'Pierce': '관통탄', 'Pellet': '산탄', 'Crag': '철갑유탄', 'Clust': '확산탄'}
COAT = {'Poison': '독병', 'Para': '마비병', 'Sleep': '수면병', 'C.Range': '접격병', 'Exhaust': '감기병', 'Blast': '폭파병'}
UNVERIFIED = '미검증'
SHARED = '같은 구성 스킬을 따로 발동해도 그 효과는 겹치지 않습니다.'


def ref(name):
    return '{{' + name + '}}'


def build(native):
    effects = dict(BASE)
    for i, tier in enumerate(('S', 'M', 'L')):
        effects[f'Attack Up ({tier})'] = f'물리 공격력 +{(10, 15, 20)[i]}(고정). 장착 중 항상 적용되며, 배율 보정보다 먼저 더해집니다.'
        effects[f'Attack Down ({tier})'] = f'물리 공격력 -{(5, 10, 15)[i]}(고정). 장착 중 항상 적용.'
        effects[f'Defense Up ({tier})'] = ('방어력 +15(고정).', '방어력 x1.03 후 +20.', '방어력 x1.06 후 +25.')[i]
        effects[f'Defense Down ({tier})'] = ('방어력 -10(고정).', '방어력 x0.95 후 -10.', '방어력 x0.90 후 -10.')[i]
    for element, k in EL.items():
        for level, percent, flat in [(1, 5, 4), (2, 10, 6)]:
            effects[f'{element} Atk +{level}'] = (f'{k} 속성치 x{1 + percent / 100:.2f} 후 +{flat}. 무기에 이미 그 속성이 있을 때만 적용. '
                                                  f'{ref("Element Atk Up")}과 합친 배율은 x1.20이 상한입니다.')
        effects[f'{element} Atk Down'] = f'{k} 속성이 있는 무기·탄의 {k} 속성치 x0.75. 다른 속성에는 영향이 없습니다.'
        for value in (20, 15, -20):
            effects[f'{element} Res {value:+}'] = (f'{k} 속성 내성 {value:+}(장착 중). 내성 1마다 받는 {k} 속성 피해가 1%p 바뀝니다. '
                                                   '물리 피해는 바뀌지 않습니다.')
    for health in (50, 20, -10, -30):
        effects[f'Health {health:+}'] = f'최대 체력 {health:+}. 체력 상한 150을 넘지 않으며, 음식·아이템과 합쳐도 상한을 넘지 않습니다.'
    for level, affinity in [(1, 10), (2, 20), (3, 30), (-1, -5), (-2, -10), (-3, -15)]:
        effects[f'Critical Eye {level:+}'] = f'회심률 {affinity:+}%p(장착 중). 최종 회심률은 +100%가 상한입니다.'
    for level in (1, 2, 3, -1, -2, -3):
        effects[f'Reload Speed {level:+}'] = (f'보우건 장전 속도 {level:+}단계. 실제 장전 시간은 보우건과 탄의 기준에 따라 다릅니다. '
                                              + ('활의 병은 자동으로 장착됩니다.' if level >= 2 else '활의 병 장착이 빨라집니다.' if level == 1 else '활의 병 장착이 느려집니다.'))
        effects[f'Recoil Down {level:+}'] = f'보우건 반동 {-level:+}단계. 탄·보우건이 반동 기준을 넘을 때만 실제 반동 동작이 바뀝니다.'
    for level in (1, 2, -1, -2):
        effects[f'Steadiness {level:+}'] = f'보우건 편차 {-level:+}단계. +2는 원래 편차를 없애며, 사격 피해는 늘지 않습니다.'
    for kind, k in KIND.items():
        for any_level in (False, True):
            name = f'Use Any {kind} S' if any_level else f'Use Lv1 {kind} S'
            if name in native:
                effects[name] = (f'{k} ' + ('LV1~3' if any_level else 'LV1') +
                                 '을 원래 못 쓰는 보우건에서 쓸 수 있게 합니다. 장전 수는 보우건에 따르며, 탄이 공짜가 되거나 피해가 늘지는 않습니다.')
    for coating, k in COAT.items():
        effects[f'Use {coating} Coat'] = f'원래 {k}을 못 쓰는 활에서 쓸 수 있게 합니다. 이미 쓸 수 있는 활의 병 효과가 강해지지는 않습니다.'
    for kind, k in (('Power', '강격병'), ('Element', '속성강화병')):
        for tier in ('Lv1', 'All'):
            effects[f'{kind} Phial ({tier})'] = (f'활에 {k}이 없으면 ' + ('LV1과 LV2를' if tier == 'All' else 'LV1을') +
                                                 ' 쓸 수 있게 합니다. 슬래시액스·차지액스의 병 보너스가 아니라 활의 병 사용입니다.')
    for chance in (45, 20, -10, -20):
        effects[f'Combination {chance:+}%'] = f'조합 성공률 {chance:+}%p. 최종 확률은 100%가 상한입니다.'
    for name, roll in [('Miraculous Luck', 31), ('Great Luck', 28), ('Good Luck', 25), ('Bad Luck', 16), ('Horrible Luck', 8)]:
        effects[name] = (f'확정 보수 칸 이후 다음 칸이 나올 확률이 22/32(68.750%) 대신 {roll}/32({roll / 32 * 100:.3f}%)가 됩니다'
                         '(보수 칸 상한까지). 개별 아이템 확률은 그대로입니다.')
    for name, mult in [("Spirit's Whim", 2 / 3), ('Divine Whim', .5), ("Spectre's Whim", 4 / 3), ("Devil's Whim", 1.5)]:
        effects[name] = f'곡괭이·벌레잡이 그물·부메랑·피리·호루라기가 쓸 때마다 부서질 확률 x{mult:.3f}. 도구 자체의 파손 확률에 곱하며, 고정 확률이 아닙니다.'
    components = {}
    for name, description in native.items():
        if 'Combines the effects of' in description:
            components[name] = [v.strip(', .') for v in re.findall(r'"([^\"]+)"', description)]

    def resolve(name, trail=()):
        if name in effects:
            return effects[name]
        if name not in components:
            raise KeyError('No Korean mechanic description: ' + name)
        if name in trail:
            raise ValueError('Compound cycle: ' + name)
        effects[name] = ' '.join(ref(child) + ': ' + resolve(child, trail + (name,)) for child in components[name]) + ' ' + SHARED
        return effects[name]

    for name in native:
        resolve(name)
    return {name: effects[name] for name in native}, components


def descriptions(native):
    effects, components = build(native)

    def full(name):
        if name in components:
            return '\n'.join(ref(child) + ': ' + full(child) for child in components[name]) + '\n' + SHARED
        return FULL.get(name, effects[name])

    result = {}
    for name, text in effects.items():
        complete = full(name)
        short = text
        if len(short) > 160:
            short = ('; '.join(ref(child) + ': ' + effects[child].split('. ')[0] for child in components[name])
                     if name in components else text)
            if len(short) > 160:
                short = short[:150].rsplit(' ', 1)[0] + '…'
        if UNVERIFIED in complete and UNVERIFIED not in short:
            short += ' 일부 부가 수치는 미검증입니다. 전체 효과를 참고하세요.'
        result[name] = (short, complete)
    return result

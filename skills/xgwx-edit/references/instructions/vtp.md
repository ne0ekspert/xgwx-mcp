---
name: "xgk-vtp"
description: "Use VTP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4418vtp.htm"]
---

# VTP

Manual section(s): 4.41.8. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `VTP`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `VTP`; the catalog records opcode `962` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령을 실행시 속도 제어 운전중인 상태가 위치 제어로 전환되고, 속도 제어시 설정 된 목표위치로 위치 운전합니다.
- sl로 지정된 슬롯의 위치결정 모듈의 ax(지정된 축)에 속도/위치 제어전환하는 명령입니다.
- 운전 데이터나 직접기동의 제어방식을 “단축속도제어”로 설정하고, 속도 제어 운전 중「속도/위치 전환」명령으로 위치결정을 실행합니다.
- 목표 위치에 설정된 값의 부호에 따라 이동 방향이 결정됩니다.
- (1) 정방향 운전:위치 값이 양수(+)일 때
- (2) 역방향 운전:위치 값이 음수(-)일 때
- 속도/위치 전환 시 설정된 위치 값을 속도/위치 전환 명령이 실행된 시점에서의 상대 위치로 볼 것인지, 절대 위치 값으로 볼 것인지를 속도/위치 전환 좌표에 설정합니다.
- 속도/위치 전환 제어에서는 좌표 값의 설정은 동작에 영향을 미치지 않습니다.
- “절대, 단축속도제어”와 “상대, 단축속도제어의 동작은 일치합니다.

| 항목 | 설정 값 | 내용 |
| --- | --- | --- |
| 속도/위치 전환 좌표 | 0 :상대 | 속도/위치 전환 명령이 실행된 위치에서 설정 된 값만큼 증분된 위치로 위치결정을 실행합니다. |
| 속도/위치 전환 좌표 | 1 :절대 | 설정된 위치 값을 절대 위치로 보고, 설정된 절대 위치로 위치결정을 실행합니다. |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| VTP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 속도 제어의 운전 패턴은 “종료” 또는 “계속”으로 설정해 주십시오.
- “연속”으로 설정되어 있을 경우, 속도 제어를 실행할 수 없습니다.
- 속도/위치 전환 좌표 = 「0:상대」 인 경우 운전 데이터나 직접기동의 목표 위치 설정 값이 0인 경우에 속도/위치 전환 명령이 동작하지 않고 에러가 발생합니다.
- 이 경우 속도 제어로 계속 운전됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![VTP source figure 1](assets/6eac0ea47ed962b0-NewItem3608.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4418vtp.htm` — §4.41.8; source SHA-256 `1f56b3fe2064283117ff03302eaaf889e2630d130b52b7cf2a7d7a018acf70d8`.
- Additional behavior/operand evidence: `4418vtp.htm`.

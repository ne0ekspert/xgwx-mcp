---
name: "xgk-dst"
description: "Use DST with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4413dst.htm"]
---

# DST

Manual section(s): 4.41.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DST`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DST`; the catalog records opcode `952` and `7` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 목표위치 | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 목표속도 | DWORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 드웰타임 | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | M코드 번호 | WORD | PMK, L, Z, N, D, R | Allowed |
| 7 | n5 | 콘트롤 워드 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 XG-PM에서 운전 데이터를 사용하지 않고,보조 데이터로 위치 결정 데이터를 명령어에 설정하여 직접기동 지령을 내리는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 목표위치 n1을 목표속도n2로직접 기동하며, 목표위치에 도달 후 위치결저 완료가 되기까지 드웰시간 n3가 소요됩니다.
- n4의 M코드 설정으로 다른 보조 동작을 실행 할 수 있으며, n5콘트롤 워드의 비트별 정보를 활용하여 다양한 제어를 실행 할 수 있습니다.
- 제어워드(n5)의 Bit별 내용은 아래와 같습니다.
- 제어 워드(n5)가 h0012이면 Feed 제어, 상대, 가속시간1, 감속 시간1로 설정됩니다.
- 제어 워드(n5)의 2 ~ 3, 5 ~ 7, 12 ~ 15번째 비트는 미사용 영역으로 설정에 영향을 주지 않습니다.
- 역방향 속도 제어를 실행 하고자 할 경우, n1항목에 현재 위치보다 작은 위치값을 입력하시면 역방향으로 속도 제어가 됩니다.

| 15 ~ 12 | 11 ~ 10 | 9 ~ 8 | 7 ~ 5 | 4 | 3 ~ 2 | 1 ~ 0 |
| --- | --- | --- | --- | --- | --- | --- |
| - | 감속시간 | 가속시간 | - | 0:절대좌표 1:상대좌표 | - | 0:위치제어 1:속도제어 2:Feed제어 |

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
| DST | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DST source figure 1](assets/29fc0364836397f0-NewItem3589.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4413dst.htm` — §4.41.3; source SHA-256 `2f6d08c77241c05cf0be191bafb25492949107df7cb50e8d23644328bde817ac`.
- Additional behavior/operand evidence: `4413dst.htm`.

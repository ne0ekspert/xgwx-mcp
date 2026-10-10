---
name: "xgk-xrstr"
description: "Use XRSTR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44254xrstr.htm"]
---

# XRSTR

Manual section(s): 4.42.54. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XRSTR`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XRSTR`; the catalog records opcode `1214` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 운전 중이던 명령축을 감속정지로 멈춘 후 이전 운전 시 설정한 데이터로 다시 운전 하도록 하는 명령입니다.
- 위치결정 운전 중 정지 요인에 의해 감속 정지 하였을 경우에는 “재기동” 명령을 사용하여 정지한 위치에서 이전 운전에 사용된 운전 조건을 사용하여 목표위치까지 운전을 재개할 수 있습니다.
- 단, 이전 운전에 사용되었던 명령이 “직접기동”과 “간접기동”이었을 경우에 재기동이 유효합니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면, 에러가 발생(F110이 Set)합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XRSTR | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 변경된 사용자 캠 데이터가 반영되는 시점은 현재 동작하고 있는 사용자 캠 데이터의 한 사이클이 완료된 다음부터 입니다.
- 포인트 단위 설정 후 캠(CAM) 블록 데이터를 설정할 때, 「주축끝위치」는 포인트 단위의 정수 배로 설정해야 합니다.
- 예를 들어 주축의 단위가 “degree” 이고, 포인트 단위가 10 인 경우 「주축끝위치」는 40, 90, 180, ??? 등 10의 배수로 설정해야 합니다.
- 다음의 경우는 에러가 되어 캠 운전 명령을 실행할 수 없습니다.
- M코드 신호가 On된 상태에서 캠 운전 명령을 실행한 경우 에러(702)가 발생합니다.
- 설정된 주축이 설정 가능한 축이 아니거나, 주축 설정을 명령축과 같게 설정한 경우 에러(704)가 발생합니다.
- 주축의 속도가 빨라서 캠 운전 데이터의 종축 위치로 계산되는 종축의 속도가 속도 제한치를 벗어나는 경우, 에러(708)가 발생합니다.
- 종축의 속도가 속도 제한치를 벗어나는 경우 에러가 발생하고, 기본 파라미터의 「급정지시 감속시간」으로 감속 정지합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XRSTR source figure 1](assets/af62a2ff9c6b1685-NewItem3960.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44254xrstr.htm` — §4.42.54; source SHA-256 `c16b15911fd1eb43e65f65861441f509e793c0f68d38dbe64725d4617e238196`.
- Additional behavior/operand evidence: `44254xrstr.htm`.

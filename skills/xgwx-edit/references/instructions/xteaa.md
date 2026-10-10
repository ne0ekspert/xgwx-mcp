---
name: "xgk-xteaa"
description: "Use XTEAA with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44225xteaa.htm"]
---

# XTEAA

Manual section(s): 4.42.25. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XTEAA`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XTEAA`; the catalog records opcode `1188` and `6` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 직접 기동 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭할 선두 스텝 번호(0 ~ 400) | WORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 티칭 방법 설정 0:RAM티칭, 1:ROM티칭 | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 티칭 항목 설정 0:위치티칭, 1:속도티칭 | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | 티칭할 스텝의 개수 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 위치결정 운전 데이터에서 여러 개의 스텝에 설정한 목표 위치 또는 운전 속도의 값을 변경하는 기능입니다.
- 목표 위치 값과 운전 속도 값을 빈번하게 변경하여 사용할 경우 이 기능을 사용하면 편리합니다.
- 목표 위치 값이 변경되는 것을 위치 티칭이라 하고 운전 속도 값이 변경되는 것을 속도 티칭이라고 합니다.
- 최대 16개의 데이터를 변경할 수 있습니다.
- 티칭 할 축이 운전 중인 경우에도 실행할 수 있지만 운전 중인 스텝이 티칭 할 스텝 영역 안에 있 을 때는 현재 스텝의 데이터만 현재 스텝의 운전이 종료된 후 반영되고, 나머지 스텝의 데이터는 즉시 변경됩니다
- 변경하려는 데이터를 저장하는 위치에 따라서 RAM 티칭 기능과 ROM 티칭 기능이 있습니다.
- RAM 티칭 기능: 속도 값과 위치 값을 변경하여 사용할 수 있으나 전원이 Off 되면 속도 값과 위치 값을 상실하게 됩니다.
- ROM 티칭 기능: 속도 값과 위치 값을 변경하여 사용할 수 있으며, FRAM(비휘발성 메모리)에 운전 데이터를 저장하여 전원이 Off 되어도 기존에 사용하던 속도 값과 위치 값을 영구히 보존 가능합니다.

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
| XTEAA | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음과 같은 경우 에러가 되어 복수 티칭 명령을 실행할 수 없습니다.
- 복수 티칭 개수를 1 ~ 16 이외의 값으로 설정한 경우 (에러코드:462)
- 티칭 스텝 번호를 1 ~ 400 이외의 값으로 설정한 경우 (에러코드:465) 티칭 스텝 번호 + 티칭 개수를 400 이하로 설정하십시오.
- 복수 티칭은 티칭 할 축이 운전 중인 경우에도 실행할 수 있지만 운전 중인 스텝이 티칭 할 스텝 영역 안에 있을 때는 현재 스텝의 데이터만 현재 스텝의 운전이 종료된 후 반영되고,나머지 스텝의 데이터는 즉시 변경됩니다.
- 복수 티칭 명령을 실행하기 이전에 복수 티칭 데이터 설정 영역에 티칭 데이터가 설정되어 있어야 합니다.
- 복수 티칭 데이터 설정 명령어 XTWR을 참조하십시오.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XTEAA source figure 1](assets/dea9a855d223317c-NewItem3867.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44225xteaa.htm` — §4.42.25; source SHA-256 `32705912cdf1c02738864225e7bff4d318c71782e8a04cc908dfc11edfbfcb70`.
- Additional behavior/operand evidence: `44225xteaa.htm`.

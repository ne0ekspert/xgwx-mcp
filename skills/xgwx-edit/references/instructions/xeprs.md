---
name: "xgk-xeprs"
description: "Preset the current position of a selected encoder."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44224xeprs.htm"]
---

# XEPRS

Preset the current position of a selected encoder.

Manual section(s): 4.42.24. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XEPRS`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XEPRS`; the catalog records opcode `1155` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- sl selects the module; D is the new DINT position and n1 selects encoder1/2 as 0/1.

## Instruction constraints

- Send separate preset commands for the two encoder channels and execute only while no external encoder pulses arrive.
- A preset is rejected when the encoder is the master of an operating axis (codes 532/533) or the position exceeds its configured minimum/maximum (codes 534/535).

## Source corrections

- The initial support matrix says ZOE. The body calls the encoder selector n2, although the operand table calls it n1, and its general flag/error text refers to nonexistent ax. Use the actual sl,D,n1 operand table; do not add ax.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | D | 변경할 엔코더 위치값 | DINT | Unknown | Unknown |
| 3 | n1 | 변경할 엔코더(0:엔코더1, 1:엔코더2) | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 엔코더의 현재 값을 프리셋 지정한 값(D)으로 변경하는 명령입니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 하나의 접점으로 2개의 엔코더 채널에 프리셋 지령을 내리지 말고, 각 채널에 따로 내려주시기 바랍니다.
- 엔코더를 주축으로 운전하는 축이 있으면 종축의 속도가 급격하게 변경될 수 있으므로 엔코더 프리셋 명령을 실행할 수 없습니다.
- 외부에서 엔코더 펄스 입력이 입력되지 않는 상태에서 엔코더 프리셋 명령을 실행하여 주시기 바랍니다.
- 다음의 경우는 에러가 되어 엔코더 프리셋 명령이 실행되지 않습니다.
- (1) 엔코더1을 주축으로 운전하는 축이 있는 경우 (에러코드:532)
- (2) 엔코더2를 주축으로 운전하는 축이 있는 경우 (에러코드:533)
- (3) 엔코더1의 프리셋 위치 값이 공통 파라미터 엔코더1 최대값과 엔코더1 최소값의 범위를 벗어난 경우(에러코드:534)
- (4) 엔코더2의 프리셋 위치 값이 공통 파라미터 엔코더2 최대값과 엔코더2 최소값의 범위를 벗어난 경우(에러코드:535)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XEPRS source figure 1](assets/75319284d50f59ff-NewItem3863.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44224xeprs.htm` — §4.42.24; source SHA-256 `dd5516255307ac8bf0f2293a762c7913d34b545668d30eb0c134bafa4471e93d`.
- Additional behavior/operand evidence: `44224xeprs.htm`.

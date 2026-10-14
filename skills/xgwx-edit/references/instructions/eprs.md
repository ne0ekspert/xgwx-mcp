---
name: "xgk-eprs"
description: "Use EPRS with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44126eprs.htm"]
---

# EPRS

Manual section(s): 4.41.26. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `EPRS`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `EPRS`; the catalog records opcode `998` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 명령을 내릴 축 (0:X 축,1:Y 축,2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 변경할 엔코더 설정값(0~4294967295) | DINT | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 엔코더의 현재 값을 프리셋 지정한 값(D)으로 변경하는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 엔코더의 현재값을 n1으로 변경하는 엔코더 현재값 변경 명령을 합니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| EPRS | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 엔코더를 주축으로 운전하는 축이 있으면 종축의 속도가 급격하게 변경될 수 있으므로 엔코더 프리셋 명령을 실행할 수 없습니다.
- 외부에서 엔코더 펄스 입력이 입력되지 않는 상태에서 엔코더 프리셋 명령을 실행하여 주시기 바랍니다.
- 다음의 경우는 에러가 되어 엔코더 프리셋 명령이 실행되지 않습니다.
- (1) 엔코더를 주축으로 운전하는 축이 있는 경우
- (2) 엔코더의 프리셋 위치 값이 공통 파라미터 엔코더 최대/최소값의 범위를 벗어난 경우

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![EPRS source figure 1](assets/c3dd8bbca2ef74a5-NewItem3692.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44126eprs.htm` — §4.41.26; source SHA-256 `bb49df63783eb250bb42602d99b044efdc9ef9b132181e9af20a448cb0a6472d`.
- Additional behavior/operand evidence: `44126eprs.htm`.

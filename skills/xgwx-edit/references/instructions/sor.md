---
name: "xgk-sor"
description: "Use SOR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44115sor.htm"]
---

# SOR

Manual section(s): 4.41.15. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SOR`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `SOR`; the catalog records opcode `976` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 변경할 목표 속도값 | DWORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 현재 운전 중인 위치결정 제어의 운전 속도를 변경하고자 하는 경우에 속도 오버라이드 명령을 사용하여 운전 속도를 변경할 수 있습니다.
- 속도 오버라이드 명령은 운전 패턴 중 가속 및 정속 구간에서만 사용 가능하며 사용할 수 있는 운전 모드는종료 운전, 계속 운전, 연속 운전입니다.
- 속도 오버라이드 값의 단위는 축의 단위를 따릅니다.
- 운전 중인 경우 복수 번 실행 가능합니다.

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
| SOR | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음의 경우는 에러가 되어 속도 오버라이드를 실행하지 않고, 이전의 속도로 계속 운전합니다.
- 속도 오버라이드 값이 기본 파라미터의 속도 제한치를 초과한 경우
- 속도 오버라이드 속도 값은 속도 제한치 이하로 설정하시기 바랍니다.
- 직선보간의 종축에 대하여 속도 오버라이드 명령을 실행한 경우
- 원호보간의 종축에 대하여 속도 오버라이드 명령을 실행한 경우
- 동기운전 종축에 대하여 속도 오버라이드 명령을 실행한 경우
- 감속 구간에서 속도 오버라이드 명령을 실행한 경우

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SOR source figure 1](assets/ffbff268639a6277-NewItem3640.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44115sor.htm` — §4.41.15; source SHA-256 `2f298de4ab1820dbf6171d23a32aaa088da15010ea31c1641741da56a5a736b3`.
- Additional behavior/operand evidence: `44115sor.htm`.

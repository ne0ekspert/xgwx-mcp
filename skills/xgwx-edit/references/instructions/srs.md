---
name: "xgk-srs"
description: "Use SRS with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44121srs1.htm"]
---

# SRS

Manual section(s): 4.41.21. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SRS`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `SRS`; the catalog records opcode `988` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | Not unambiguously specified in this topic | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 해당 명령은 M코드 발생시에 M코드 신호와 M코드 번호를 동시에 해제(클리어)합니다.
- M코드 신호는 Off, M코드 번호는 O으로 변경합니다.
- 해당 명령은 운전 중이나 위치결정 완료 이후 어느 구간에서도 실행할 수 있습니다.

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
| SRS | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 스텝 번호가 0인 경우 이전의 반복 운전 스텝 번호를 그대로 유지합니다.
- 반복 운전 스텝 번호 변경 명령은 실행 시점에서 현재 운전 스텝이 변경되지 않습니다.
- 운전 방식이 “반복”으로 설정된 위치결정 데이터의 운전이 완료된 후 반복 운전 스텝 번호 변경 명령에 의해 지정된 스텝으로 변경됩니다.
- 다음의 경우는 에러가 되어 반복 운전 스텝 번호 변경 명령이 실행되지 않습니다.
- 변경할 스텝 번호를 0 ~ 400 범위를 벗어난 경우

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SRS source figure 1](assets/f43b9ac6bade45ca-NewItem3672.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44121srs1.htm` — §4.41.21; source SHA-256 `c1d0766651890de70e037828b9c05a37648e47b24280c950ad67338f1114aad0`.
- Additional behavior/operand evidence: `44121srs1.htm`.

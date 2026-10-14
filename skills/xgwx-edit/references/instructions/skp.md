---
name: "xgk-skp"
description: "Use SKP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44111skp.htm"]
---

# SKP

Manual section(s): 4.41.11. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SKP`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `SKP`; the catalog records opcode `968` and `2` operands in order.
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
- 이 명령은 현재 실행 중이던 운전 스텝을 감속정지하고, 다음 운전 스텝 번호의 운전 데이터로 변경하여 위치결정 제어를 실행합니다.
- 스킵 운전 명령은 명령 실행 후에 감속 정지하고 다음 스텝의 운전을 계속하게 됩니다.
- 운전 모드가 종료, 계속, 연속 운전일 경우와 운전 패턴 중 가속, 정속, 감속구간에서 사용할 수 있습니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| SKP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음 스텝의 운전 데이터가 설정이 안된 상태에서 스킵 운전 명령을 실행하면 에러가 발생합니다.
- 아래의 경우에는 에러가 되어 연속 운전 명령을 실행하지 않고, 이전의 동작을 계속 합니다.
- 직선보간의 종축에 대하여 스킵 운전 명령을 실행한 경우
- 직선보간 운전 중인 경우의 스킵 운전 명령은 주축에 실행해야 합니다.
- 동기운전 종축에 대하여 스킵 운전 명령을 실행한 경우
- 조그 운전 중인 축에 대하여 스킵 운전 명령을 실행한 경우
- 현재 운전 중인 축이 직접 기동으로 실행된 경우
- 인칭 운전 중인 축에 대하여 스킵 운전 명령을 실행한 경우
- 원호보간의 종축에 대하여 스킵 운전 명령을 실행한 경우
- 원호보간 운전 중인 경우의 스킵 운전 명령은 주축에 실행해야 합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SKP source figure 1](assets/cd018039a2c9e4ae-NewItem3617.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44111skp.htm` — §4.41.11; source SHA-256 `1aab8e5ab2d7331184998e3f649584d032cf87922981421d31957f54022ae407`.
- Additional behavior/operand evidence: `44111skp.htm`.

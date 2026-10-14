---
name: "xgk-xpsoex"
description: "Use XPSOEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44216xpso.htm"]
---

# XPSOEX

Manual section(s): 4.42.16. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XPSOEX`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XPSOEX`; the catalog records opcode `1707` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 속도 변경을 실행할 위치값 | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 변경할 목표 속도값 | DWORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 위치 지정 속도 오버라이드 명령은 위치결정 운전 중 지정된 위치에 도달하면 설정된 운전 속도로 속도 변경되어 운전을 합니다.
- 위치 지정 속도 오버라이드 명령은 운전 패턴 중 가속 및 정속 구간에서만 사용 가능하며 사용할 수 있는 운전 모드는 종료 운전, 계속 운전, 연속 운전입니다.
- 공통 파라미터의 「속도 오버라이드」에 속도 오버라이드 값을 “%지정” 또는 “속도지정”으로 설정할 수 있습니다.
- 확장 파라미터의 「위치지정 속도 오버라이드 좌표」에서 지정된 위치 값을 절대 위치로 볼 것인지, 상대 위치로 볼 것인지 선택할 수 있습니다.
- 관련 파라미터 설정 (공통 파라미터)
- 관련 파라미터 설정 (확장 파라미터)
- 위치지정 속도 오버라이드 명령의 보조 데이터 설정

| 항목 | 설정 값 | 내용 |
| --- | --- | --- |
| 속도 오버라이드 | 0:%지정 | 속도 오버라이드 설정 값을 설정 속도의 %로 지정합니다. |
| 속도 오버라이드 | 1:속도지정 | 속도 오버라이드 설정값을 운전 속도로 직접 지정합니다. |

| 항목 | 설정 값 | 내용 |
| --- | --- | --- |
| 위치 지정 속도 오버라이드 좌표 | 0:절대 | 설정된 절대 위치에서 속도 오버라이드 됩니다. |
| 위치 지정 속도 오버라이드 좌표 | 1:상대 | 현재 운전이 시작 위치에서 설정된 값만큼 증분 된 위치에서 실행 됩니다. |

| 항목 | 설정 값 | 내용 |
| --- | --- | --- |
| 속도 | 1 ~ 65535 (1=0.01%) | 속도 오버라이드가 “%지정”인 경우 설정 속도에 대한 %를 지정합니다. (100%의 경우 10000 설정) |
| 속도 | 1 ~ 속도 제한치 | 속도 오버라이드가 “속도지정”인 경우 직접 운전 속도를 설정합니다. |

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
| XPSO | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 현재 위치가 속도 오버라이드 위치에 설정된 값과 정확하게 일치하지 않더라도, 속도 오버라이드 위치가 이전 스캔의 위치와 현재 스캔의 위치 사이에 있으면 설정된 속도로 속도 오버라이드가 실행됩니다
- 다음의 경우는 에러가 되어 위치지정 속도 오버라이드를 실행하지 않고, 이전의 속도로 계속 운전합니다.
- 현재 실행 중인 운전이 위치 결정 (단축위치제어, 인칭운전) 제어가 아닌 경우 (에러코드:382)
- 속도 오버라이드 값이 기본 파라미터의 속도 제한치를 초과한 경우 (에러코드:383) 속도 오버라이드 속도값은 속도 제한치 이하로 설정 하시기 바랍니다.
- 직선 보간 제어의 속도 오버라이드는 보간 축 각각에 대해 속도 제한치 이하가 되게 설정하시기 바랍니다.
- 직선보간의 종축에 대하여 위치지정 속도 오버라이드 명령을 실행한 경우 (에러코드:384) 직선보간 운전 중인 경우의 위치지정 속도 오버라이드 명령은 주축에 실행해야 합니다.
- 원호보간의 종축에 대하여 위치지정 속도 오버라이드 명령을 실행한 경우 (에러코드:385)원호보간 운전 중인 경우의 위치지정 속도 오버라이드 명령은 주축에 실행해야 합니다.
- 동기운전 종축에 대하여 위치지정 속도 오버라이드 명령을 실행한 경우 (에러코드:386)
- 확장 파라미터의 가감속 패턴이 「S-커브운전」인 경우 (에러코드:389)
- 감속 구간에서 위치지정 속도 오버라이드 명령을 실행한 경우 에러는 발생하지 않지만, 속도 오버라이드를 실행하지 않습니다.
- 단, 감속 구간이 아닐 때 위치지정 속도 오버라이드 명령을 실행하여, 지정한 위치가 되어 속도 오버라이드를 하려고 했을 때가 감속 중이라면 에러(에러코드:377)이 발생합니다

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XPSOEX source figure 1](assets/52238d9ba31f5b0f-NewItem3839.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44216xpso.htm` — §4.42.16; source SHA-256 `4042da567727761c940a9c78b01740ca6eddefcf71247028ea8f1ae1dfbbaa25`.
- Additional behavior/operand evidence: `44216xpso.htm`.

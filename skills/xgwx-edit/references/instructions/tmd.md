---
name: "xgk-tmd"
description: "Use TMD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44139tmd1.htm"]
---

# TMD

Manual section(s): 4.41.39. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TMD`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `TMD`; the catalog records opcode `1031` and `5` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 기본 파라미터 티칭을 실행할 지령 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭 데이터 (공통 운전 파라미터 중 변경할 항목의 변경값) | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 운전 파라미터 중 변경할 항목 (1 ~ 11, hFF) | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 변경 할 운전 데이터의 스텝 번호 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 지령축의 운전 데이터 항목 중 특정 항목의 값을 설정 한 값으로 티칭하는 명령입니다
- 해당 지령은 지정 축 n3(변경할 운전 데이터 스텝)의 n2(운전 데이터 중 변경할 항목)의 값을 n1(설정 값)으로 변경하는 명령입니다
- 운전 데이터 항목에 설정할 수 있는 값은 아래와 같습니다.

| 항목 | 항목 | 설정값 |
| --- | --- | --- |
| 1 | 목표 위치 | -2147483648 ~ 2147483647 |
| 2 | 원호 보간 보조 위치 | -2147483648 ~ 2147483647 |
| 3 | 운전 속도 | 0 ~ 최대 속도(속도 제한치) |
| 4 | 드웰 시간 | 0 ~ 50000 |
| 5 | M 코드 번호 | 0 ~ 65535 |
| 6 | 제어 방식 | 0:위치 1:속도 |
| 7 | 운전 방식 | 0:단독 1:반복 |
| 8 | 운전 패턴 | 0:종료 1:계속 2:연속 |
| 9 | 좌표 | 0:절대 1:상대 |
| 10 | 가/감속 번호 | 1 ~ 4 |
| 11 | 원호 보간 방향 | 0:CW 1:CCW |

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
| TMD | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 운전 데이터 티칭 명령으로 수정한 파라미터 값은 전원이 ON 되어 있는 동안에만 유효합니다.
- 공통 파라미터 티칭 명령으로 수정한 파라미터 값을 플래시 메모리에 저장하려면 운전 데이터 티칭 후파라미터/운전데이터 저장 명령 WRT 를 사용하여 수정한 파라미터 값을 플래시 메모리에 저장해야 합니다.
- 복수 티칭 데이터 개수에 따라 여러 스캔이 소요될 수 있습니다.
- 티칭 완료 플래그는 존재하지 않습니다. 따라서 복수개의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해 주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TMD source figure 1](assets/2a7d4b837f0ae7a1-NewItem3744.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44139tmd1.htm` — §4.41.39; source SHA-256 `cf29ba25b6f38860809784bd768df895d4f002d3d5fd37e894191acf190c50c6`.
- Additional behavior/operand evidence: `44139tmd1.htm`.

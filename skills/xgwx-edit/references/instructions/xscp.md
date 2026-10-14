---
name: "xgk-xscp"
description: "Use XSCP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44235xscp.htm"]
---

# XSCP

Manual section(s): 4.42.35. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSCP`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XSCP`; the catalog records opcode `1186` and `5` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 기본 파라미터 티칭을 실행할 지령 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭 데이터 (공통 운전 파라미터 중 변경할 항목의 변경값) | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 공통 운전 파라미터 중 변경할 항목 (1 ~ 10, 255) | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 티칭 방법 설정 (0:RAM티칭 혹은 1:ROM티칭) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 공통 파라미터 항목 중 특정 항목의 값을 설정 한 값으로 티칭하는 명령입니다
- 해당 지령은 지정 축의 n2(수동 운전 파라미터 항목 중 변경할 항목)의 값을 n1(설정 값)으로 변경하는 명령입니다
- n3항목을 0:RAM설정을 한 경우에는 변경한 값이 위치결정 모듈에 전원을 인가한 동안만 유지되며, 1:ROM설정을 한경우에는 위치결정 모듈에 전원을 인가하지 않았을 때도 유지가 됩니다.
- 위치결정 모듈의 기본 파라미터는 FRAM에 저장되므로 ROM설정의 실행 횟수는 제한이 없습니다.
- 공통 파라미터 항목은 아래와 같습니다.

| 설정값 | 항목 | 설정범위 | 초기값 |
| --- | --- | --- | --- |
| 1 | 속도 오버라이드 방식 | 0 :% 지정, 1 :속도 지정 | 0:%지정 |
| 2 | 엔코더 펄스 입력모드 | 0:CW/CCW 1체배, 1:PULSE/DIR 1체배, 2:PULSE/DIR 2체배, 3:PHASE A/B 1체배, 4:PHASE A/B 2체배, 5:PHASE A/B 4체배 | 3:PHASE A/B 1체배 |
| 3 | 엔코더 최대값 | -2147483648 ~ 2147283647 | 2,147,483,647 |
| 4 | 엔코더 최소값 | -2147483648 ~ 2147283647 | -2,147,483,648 |
| 5 | 펄스 출력 레벨 | 0 :Low Active, 1 :High Active | 0:Low Active |
| 6 | 비상정지시 원점결정상태 | 0:이전상태 유지, 1:원점미결정 | 0:이전상태 유지 |
| 7 | 위치지정 속도동기 목표위치 좌표 | 0:상대, 1:절대 | 0:상대 |
| 8 | 엔코더 평균 회수 | 0:적용안함, 1:5회, 2:10회, 3:20회 | 0:적용안함 |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XSCP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 공통 파라미터의 전체 항목을 한번의 XSCP 명령으로 모두 설정하려면 n2 파라미터 항목에 hFF(255)를 적용 할 수 있습니다.
- 공통 파라미터의 항목들은 복수 티칭 메모리 번지에 먼저 저장되어 있어야 합니다. 데이터는 PUT명령 또는 XTWR 명령으로 설정할 수 있습니다.
- 복수 티칭에 대한 설정 예제는 XSEP명령어를 참고하시기 바랍니다.
- 티칭 완료 플래그는 존재하지 않습니다.
- 복수의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSCP source figure 1](assets/a07374b17d7da34a-NewItem3900.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44235xscp.htm` — §4.42.35; source SHA-256 `86b701382b03fd5ac996910d9abd1da110d9aa2f292604bfeb3815f6918945ee`.
- Additional behavior/operand evidence: `44235xscp.htm`.

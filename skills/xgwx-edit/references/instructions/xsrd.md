---
name: "xgk-xsrd"
description: "Use XSRD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44238xsrd.htm"]
---

# XSRD

Manual section(s): 4.42.38. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSRD`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XSRD`; the catalog records opcode `1157` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 현재 상태값을 읽어서 저장할 디바이스 선두 번호 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 명령 축 운전 상태를 읽어서 설정한 디바이스에 저장하도록 하는 명령입니다.
- 해당 명령어는 다른 위치결정 명령과 달리, 상승 Edge 지령 인식 방식이 아닌 Level 방식 명령어 입니다.
- 지정된 디바이스의 선두 번호에 따라 아래와 같은 형태로 현재 상태가 저장됩니다. (모델별 상세 상태 정보는 각 기종별 사용자 설명서를 참고하시기 바랍니다.)
- GET명령을 사용하면 필요한 상태 종류만 읽어올 수 있습니다.
- 상세 주소 정보는 각각의 모듈별 사용자 설명서의 5장 상태 정보 메모리 번지를 참고하시기 바랍니다.

| 디바이스 번호 | Size | 상태종류 | 상태종류 |
| --- | --- | --- | --- |
| 디바이스 번호 | Size | XGF-PN8A | XGF-PN4B/PN8B |
| 디바이스 | WORD | 운전 상태 정보(하위) | 운전 상태 정보(하위) |
| 디바이스 + 1 | WORD | 운전 상태 정보(상위) | 운전 상태 정보(상위) |
| 디바이스 + 2 | WORD | 축정보 | 축정보 |
| 디바이스 + 3 | WORD | 외부 입력신호 상태 | 외부 입력신호 상태 |
| 디바이스 + 4 | DINT | 현재위치 | 현재위치 |
| 디바이스 + 6 | DWORD | 현재속도 | 현재속도 |
| 디바이스 + 8 | WORD | 스텝번호 | 스텝번호 |
| 디바이스 + 9 | WORD | M코드 번호 | M코드 번호 |
| 디바이스 + 10 | WORD | 에러정보 | 에러정보 |
| 디바이스 + 11 ~ 디바이스 + 17 | WORD | 에러히스토리 1 ~ 7 | 에러히스토리 1 ~ 7 |
| 디바이스 + 18 | WORD | 에러히스토리 8 | 외부입력신호 하위 |
| 디바이스 + 19 | WORD | 에러히스토리 9 | 외부입력신호 상위 |
| 디바이스 + 20 | WORD | 서보 에러 정보 | 서보 에러 정보 |
| 디바이스 + 21 | DINT | 엔코더1값 | 엔코더1값 |
| 디바이스 + 23 | DINT | 엔코더2값 | 엔코더2값 |
| 디바이스 + 25 | DINT | 명령 위치 | 명령 위치 |
| 디바이스 + 27 | DWORD | 명령 속도 | 명령 속도 |
| 디바이스 + 29 | WORD | 토크 | 토크 |
| 디바이스 + 30 | WORD | 공통에러정보 | 공통에러정보 |

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
| XSRD | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- XSRD는 하이 레벨(High level)로 동작합니다. 즉, 입력 조건이 On 인 동안 계속 동작을 실행하고,입력 조건이Off 되면 동작을 실행하지 않습니다.
- 일반적으로 위치결정 모듈 전용 명령어는 상승 에지(Rising edge)에서 동작합니다.
- 입력 조건이 On 될 때 처음 한번만 동작을 실행하고,다시 동작을 실행하려면 입력 조건이 Off 되었다가 다시 On 되어야 합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSRD source figure 1](assets/5959ff319802296d-NewItem3906.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44238xsrd.htm` — §4.42.38; source SHA-256 `989f37397fcc801d9d5f0a1b4658c0d40c22d8d6464e3fc5e12f309e00ddb989`.
- Additional behavior/operand evidence: `44238xsrd.htm`.

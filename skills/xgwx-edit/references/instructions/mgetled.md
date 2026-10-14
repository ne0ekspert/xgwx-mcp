---
name: "xgk-mgetled"
description: "Use MGETLED with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44015mgetled.htm"]
---

# MGETLED

Manual section(s): 4.40.15. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `MGETLED`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `MGETLED`; the catalog records opcode `1747` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | FENET 모듈이 장착된 베이스 및 슬롯번호 | WORD | Unknown | Unknown |
| 2 | D1 | LED 정보를 저장할 디바이스 번호 | DWORD | Unknown | Unknown |
| 3 | D2 | 상태 코드 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

Reviewed XGK model table: XGK-CPUUN, XGK-CPUHN, XGK-CPUSN (`44015mgetled.htm`).
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- FENET 모듈의 LED 정보를 읽을 수 있는 명령어입니다.
- Sl은 현재 FENET 모듈이 장착된 베이스와 슬롯 번호를 입력합니다.
- EX. 3번 베이스 5번 슬롯일 경우 : h35
- D1의 디바이스 번호가 M0인 경우 읽은 모듈의 동작 상태에 따라 LED 정보는 아래와 같이 표현됩니다.
- EX. 고속링크 설정, LNK0 ON, RUN 일 경우 : h00001201
- LED 정보는 아래의 LED bitmap을 참조 해주세요.
- Bitmap LED 순서
- D2는 명령어 정보를 저장하는 임시 영역으로, 저장되는 값은 아래와 같습니다. 초기값은 모두 0이며, 사용자가 데이터 값을 조작할 경우 명령어가 오동작 할 수 있습니다.
- 상태 정보

| Number | LED 정보 | Number | LED 정보 |
| --- | --- | --- | --- |
| 0 | HS | 8 | ACT0 |
| 1 | P2P | 9 | LINK0 |
| 2 | PADT | 10 | ACT1 |
| 3 | ERR | 11 | LINK1 |
| 4 | SVR | 12 | RUN |
| 5 | RELAY | 13 | - |
| 6 | CHK | 14 | - |
| 7 | FAULT | 15 | - |

| CPU영역 | 데이터 크기 | 상태종류 |
| --- | --- | --- |
| D2 | WORD | 에러 코드 |
| D2+1 | WORD | 내부 상태(WAIT or DONE) |
| D2+2 | WORD | RETRY COUNT |

| 상태코드 | 상태 정보 | 의미 |
| --- | --- | --- |
| 00 | 초기 상태 | 명령어 수행 전 |
| 01 | 완료 | 명령어가 정상적으로 수행된 경우 |
| 02 | 모듈설정 에러 | sl값이 FENET 모듈이 장착되지 않은 베이스 또는 슬롯으로 설정된 경우 |
| 03 | 버전 호환 오류 | FEnet버전이 V8.3미만으로 해당 명령어를 지원하지 않는 경우 (CPU 버전이 V1.5 미만 인 경우는 프로그램 다운로드가 되지 않습니다.) |
| 05 | 타임아웃 에러 | FENET모듈 이상으로 명령어에 대한 응답이 없는 경우 |
| 06 | 이전 명령 수행 중 | 같은 슬롯에 대해서 이전 명령의 수행이 완료되기 전에 기동 조건을 만났을 때 (FENET 모듈의 IP/MAC 정보를 읽는 명령어가 수행 중일 경우도 발생할 수 있습니다.) |
| 07 | Buffer Full 에러 | 요청 메모리 버퍼가 Full 인 경우 |
| 08 | IO 스킵 설정 오류 | 사용자가 해당모듈에 스킵을 걸어둔 상태 |
| 09 | 모듈 탈락 오류 | 해당 모듈이 수행 중 탈락되는 상태 |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 에러 코드 확인 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XG5000 V4.51, XGK V1.50

| 지원 기종 | XGK | XGK | XGK | XGK | XGK | XGK-N | XGK-N | XGK-N | XBC-U | XBC-U | XBC-U | XBC-U | XBM | XBM |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 지원 기종 | U | H | A | S | E | UN | HN | SN | U | H | SU | E | S | H(P) |
| 지원 기종 | X | X | X | X | X | O | O | O | X | X | X | X | X | X |

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| MGETLED | - | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![MGETLED source figure 1](assets/dab89969093569a2-NewItem3544.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44015mgetled.htm` — §4.40.15; source SHA-256 `a8554e1e2b74ab5dca992ad5f11dc5cb87ed1805ac8db369fee8554f87a92529`.
- Additional behavior/operand evidence: `44015mgetled.htm`.

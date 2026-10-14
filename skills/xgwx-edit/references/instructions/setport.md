---
name: "xgk-setport"
description: "Use SETPORT with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44019setport.htm"]
---

# SETPORT

Manual section(s): 4.40.19. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SETPORT`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `SETPORT`; the catalog records opcode `1741` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 설정할 채널 번호가 저장된 디바이스 영역 | WORD | PMK, L, Z, U, N, D, R | Allowed |
| 2 | S2 | 설정할 포트 번호가 저장된 디바이스 영역 | WORD | PMK, L, Z, U, N, D, R | Allowed |
| 3 | D | 상태코드가 저장될 디바이스 영역 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

Reviewed XGK model table: no XGK model is allowed (`44019setport.htm`).
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 이 명령은 로컬이더넷의 포트를 설정할 수 있는 명령어입니다.
- 사용조건: 아래같은 P2P 설정 값이 PLC에 적용되어 있어야 합니다.
- (1) P2P 드라이버: 사용자 프레임 정의
- (2) TCP/UDP: TCP
- (3) 포트번호: 사용자 지정 값
- (4) 상대국 IP 주소: 사용자 지정 값
- S1, S2에 설정된 디바이스 영역에 설정할 로컬이더넷의 채널, 포트번호를 입력합니다.
- D에 설정된 디바이스 영역에는 상태 정보가 저장됩니다.
- (1) XBC-DxxxU: Ver 2.1 이상
- (2) XBC-DN32H: Ver 1.3 이상
- (3) XBC-Dx32HP/H2 Ver: 2.2 이상

| 상태코드 | 상태 정보 | 의미 |
| --- | --- | --- |
| 0 | 초기상태 | 명령어 수행 전 |
| 1 | 완료 | 명령어가 정상적으로 수행된 경우 |
| 2 | 채널 설정 에러 | 채널 설정 범위를 벗어난 경우 (정상범위: 0 ~ 14) |
| 3 | 드라이버 설정 에러 | P2P 드라이버가 사용자 프레임 정의가 아닌 경우 |
| 4 | 포트 설정 에러 | 포트 설정 범위를 벗어난 경우 (정상범위: 1 ~ 65535) |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 채널 설정 에러, 포트 설정 등이 잘못된 경우 (상태코드 2~4 발생 시) | F110 |

## Manual applicability and restrictions

- 적용제품: XGB 로컬이더넷 지원 기본유닛(XBL-EMTA에서는 사용할 수 없습니다.)

| 지원 기종 | XGK | XGK | XGK | XGK | XGK | XGK-N | XGK-N | XGK-N | XBC-U | XBC-U | XBC-U | XBC-U | XBM | XBM |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 지원 기종 | U | H | A | S | E | UN | HN | SN | U | H | SU | E | S | H(P) |
| 지원 기종 | X | X | X | X | X | X | X | X | O XBC-U: V2.3 | X | X | X | X | O XBM-H: V1.3 XBM-HP: V2.2 XBM-H2: V2.2 |

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| SETPORT | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SETPORT source figure 1](assets/aab11ad23100fc49-NewItem3569.png)

![SETPORT source figure 2](assets/f326e31165b2c127-NewItem3570.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44019setport.htm` — §4.40.19; source SHA-256 `6f92a530e7998ea9e88840e79c8e0ea0047e12ca1575387b211e266c3e2419a2`.
- Additional behavior/operand evidence: `44019setport.htm`.

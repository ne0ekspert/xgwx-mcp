---
name: "xgk-daterd2"
description: "Use DATERD2 with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4291daterddaterdp.htm"]
---

# DATERD2

Manual section(s): 4.29.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DATERD2`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DATERD2`; the catalog records opcode `1784` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- DATERD2 (Date Read)
- PLC의 날짜와 시간 데이터를 읽어와서 D에 저장합니다.
- 모든 시간 데이터 값은 BCD형태로 제공됩니다.
- 위 그림에서 ‘백년’은 년도에서 천단위와 백단위를 나타냅니다.
- 예를 들어, 현재 PLC시계가 2004년 10월 15일 14시 59분 40초 금요일이라면, 다음과 같은 결과가 저장됩니다.
- 요일 설명 : 0-일요일, 1-월요일, 2-화요일, 3-수요일, 4-목요일, 5-금요일, 6-토요일

## Manual applicability and restrictions

- 적용 기종 : XGK-N

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DATERD(P) | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DATERD2 source figure 1](assets/f148f7ae982ee389-NewItem4375.png)

![DATERD2 source figure 2](assets/b09d8ae8170bb358-NewItem4378.png)

![DATERD2 source figure 3](assets/9d69d0f94875fe2b-NewItem4379.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4291daterddaterdp.htm` — §4.29.2; source SHA-256 `7a1277e8a2ca0e22bae85bebb95831513974850cf931a4f905412c34f956f750`.
- Additional behavior/operand evidence: `4291daterddaterdp.htm`.

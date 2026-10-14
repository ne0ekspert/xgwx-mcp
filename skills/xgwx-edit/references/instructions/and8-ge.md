---
name: "xgk-and8-ge"
description: "Combine the current circuit result with an AND comparison: AND8>=."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41517and4xand8x.htm"]
---

# AND8>=

Combine the current circuit result with an AND comparison: AND8>=.

Manual section(s): 4.15.17. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `AND8>=`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_comparison` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `8>=`; the catalog records opcode `1423` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare S1 >= S2 as 8-bit unsigned bit field; the comparison is On when the relation holds and Off otherwise.
- Combine the comparison result with the current bit result (BR) using AND.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 비교하게 되는 데이터나 데이터 주소 | BYTE | PMK, L, D.x, R.x, U | Allowed |
| 2 | S2 | S1과 비교하게 되는 데이터나 데이터 주소 | BYTE | PMK, L, D.x, R.x, U | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- AND4 X ( =, >, <, >=, <=, <>)
- S1 과 S2를 NIBBLE단위로 비교하여 X조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 BR값을 AND하여 새로운 연산결과로 취합니다.
- S1 과 S2의 비교는Unsigned연산으로 실행합니다.
- AND8 X ( =, >, <, >=, <=, <>)
- S1 과 S2를 BYTE단위로 비교하여 X조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 BR값을 AND하여 새로운 연산결과로 취합니다.

| X조건 | 조건 | 연산결과 |
| --- | --- | --- |
| = | S1 = S2 | On |
| <= | S1 ￡ S2 | On |
| >= | S1 ³ S2 | On |
| <> | S1 ¹ S2 | On |
| < | S1 < S2 | On |
| > | S1 > S2 | On |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| AND4 X AND8 X | 3~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![AND8>= source figure 1](assets/ba4db1979f9c653a-NewItem2842.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41517and4xand8x.htm` — §4.15.17; source SHA-256 `1faca989eedc394bfa5571b297b60fb9b103dd5217f81c1d7b8d8de890edf844`.
- Additional behavior/operand evidence: `344.htm`, `41516load4xload8x.htm`.

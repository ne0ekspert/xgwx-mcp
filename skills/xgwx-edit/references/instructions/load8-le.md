---
name: "xgk-load8-le"
description: "Load an unsigned bit field comparison with relation <=."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41516load4xload8x.htm"]
---

# LOAD8<=

Load an unsigned bit field comparison with relation <=.

Manual section(s): 4.15.16. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LOAD8<=`.

## Editing route

Use `insert_ladder_comparison` after calling `describe_operation` for its current schema.
The writer mnemonic is `8<=`; the catalog records opcode `1425` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare S1 <= S2 as 8-bit unsigned bit field; the comparison is On when the relation holds and Off otherwise.
- Replace the current bit result (BR) with the comparison result.

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

- LOAD4 X (=, >, <, >=, <=, <>)
- S1 과 S2를 NIBBLE단위로 비교하여 X조건과 일치하면 현재의 연산결과를 On합니다. 이외의 연산결과는 Off합니다.
- S1 과 S2의 비교는Unsigned연산으로 실행합니다.
- LOAD8 X (=, >, <, >=, <=, <>)
- S1 과 S2를 BYTE 단위로 비교하여 X조건과 일치하면 현재의 연산결과를 On합니다. 이외의 연산결과는 Off합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| LOAD4 X LOAD8 X | 3~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LOAD8<= source figure 1](assets/48fbaf1328987838-NewItem2839.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41516load4xload8x.htm` — §4.15.16; source SHA-256 `683c8de224322711bc19249713324d1e46f3bc132fda2bc181fae854a20a80b2`.
- Additional behavior/operand evidence: `344.htm`, `41516load4xload8x.htm`.

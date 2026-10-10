---
name: "xgk-load-eq-3"
description: "Load a signed integer comparison with relation =."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41513load3xloadd3x.htm"]
---

# LOAD=3

Load a signed integer comparison with relation =.

Manual section(s): 4.15.13. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LOAD=3`.

## Editing route

Use `insert_ladder_comparison` after calling `describe_operation` for its current schema.
The writer mnemonic is `=3`; the catalog records opcode `1362` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare three 16-bit signed integers. S1, S2 and S3 must all be equal.
- Replace the current bit result (BR) with the comparison result.

## Source corrections

- The flag subsection mentions N overflow and F110, although this instruction has no N operand and the flag matrix marks no flags. Treat this as a copied subsection, not an additional flag rule.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 비교할 데이터 혹은 비교할 데이터를 지정하는 디바이스 번호 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | 비교할 데이터 혹은 비교할 데이터를 지정하는 디바이스 번호 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | S3 | 비교할 데이터 혹은 비교할 데이터를 지정하는 디바이스 번호 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| LOAD3 X LOADD3 X | 4~5 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LOAD=3 source figure 1](assets/75dc3245a8fa1b38-NewItem2834.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41513load3xloadd3x.htm` — §4.15.13; source SHA-256 `8c77cabad1a173e86a84fb1e80bb2f4d17a07821d5e90f1cea5b2fa307970efd`.
- Additional behavior/operand evidence: `344.htm`, `41513load3xloadd3x.htm`.

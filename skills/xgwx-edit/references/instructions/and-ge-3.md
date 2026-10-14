---
name: "xgk-and-ge-3"
description: "Combine the current circuit result with an AND comparison: AND>=3."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41514and3xandd3x1.htm"]
---

# AND>=3

Combine the current circuit result with an AND comparison: AND>=3.

Manual section(s): 4.15.14. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `AND>=3`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_comparison` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `>=3`; the catalog records opcode `1368` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare three 16-bit signed integers. S1 >= S2 >= S3 must hold in that order.
- Combine the comparison result with the current bit result (BR) using AND.

## Source corrections

- The initial device/support matrix labels LOAD3/LOADD3 rather than AND3/ANDD3; use the AND behavior body and reviewed schema.

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

## Manual behavior notes (Korean)

- AND3 X (=3, >3, <3, >=3, <=3, <>3)
- 비교 데이터로 지정된 S1, S2, S3로 지정된 3개의 워드 데이터를 X조건으로 비교하여 조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 연산결과를 AND하여 새로운 연산결과로 취합니다.
- S1 과 S2의 비교는Signed연산으로 실행합니다.
- h8000(-32,768) ~ hFFFF(-1) < 0 ~ h7FFF(32,767)와 같은 결과를 취하게 됩니다.
- ANDD3 X (D=3, D>3, D<3, D>=3, D<=3, D<>3)
- 비교 데이터로 지정된 (S1+1,S1), (S2+1,S2), (S3+1,S3)로 지정된 3개의 워드 데이터를 X조건으로 비교하여 조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 연산결과를 AND하여 새로운 연산결과로 취한다.
- h80000000(-2,147,483,648) ~ hFFFFFFFF(-1) < 0 ~ h7FFFFFFF(2,147,483,647)와 같은 결과를 취하게 된다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![AND>=3 source figure 1](assets/75dc3245a8fa1b38-NewItem2834.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41514and3xandd3x1.htm` — §4.15.14; source SHA-256 `d577e3c0dd41a190fe08bc2d8a9f6e0cd09d5ba8e0f3048fedcb3d0b5b3c256b`.
- Additional behavior/operand evidence: `344.htm`, `41513load3xloadd3x.htm`.

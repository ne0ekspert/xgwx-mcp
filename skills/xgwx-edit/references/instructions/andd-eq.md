---
name: "xgk-andd-eq"
description: "Combine the current circuit result with an AND comparison: ANDD=."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4152andxanddx.htm"]
---

# ANDD=

Combine the current circuit result with an AND comparison: ANDD=.

Manual section(s): 4.15.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ANDD=`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_comparison` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `D=`; the catalog records opcode `1255` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare S1 = S2 as 32-bit signed integer; the comparison is On when the relation holds and Off otherwise.
- Combine the comparison result with the current bit result (BR) using AND.

## Source corrections

- The S2 operand description mentions a transfer destination; the body defines S2 as the second comparison source.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | Not unambiguously specified in this topic | DINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | Not unambiguously specified in this topic | DINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- AND X ( =, >, <, >=, <=, < >)
- S1 과 S2를 비교하여 X조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 BR값을 AND하여 새로운 연산결과로 취합니다.
- S1 과 S2의 비교는Signed연산으로 실행합니다. 따라서 h8000(-32,768) ~ hFFFF(-1) < 0 ~ h7FFF(32,767)와 같은 결과를 취하게 됩니다.
- ANDD X (D=, D>, D<, D>=, D<=, D<>)
- S1 과 S2를 비교하여 X조건과 일치하면 On, 불일치하면 Off하고 이 결과와 현재의 BR값을 AND하여 새로운 연산결과로 합니다.
- S1과 S2의 비교는Signed연산으로 실행합니다. 따라서 h80000000(-2,147,483,648) ~ hFFFFFFFF(-1) < 0 ~ h7FFFFFFF(2,147,483,647)와 같은 결과를 취하게 됩니다.

| X조건 | 조건 | 연산결과 |
| --- | --- | --- |
| = | S1 = S2 | On |
| <= | S1 ￡ S2 | On |
| >= | S1 ³ S2 | On |
| < > | S1 ¹ S2 | On |
| < | S1 < S2 | On |
| > | S1 > S2 | On |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| AND X ANDD X | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ANDD= source figure 1](assets/8a9847e6afffa4a7-NewItem2802.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4152andxanddx.htm` — §4.15.2; source SHA-256 `d5e86aa855172f1ef19716299c1dc0dc6ee9268263c45321b9cdbdb282388f09`.
- Additional behavior/operand evidence: `344.htm`, `4151loadxloaddx.htm`.

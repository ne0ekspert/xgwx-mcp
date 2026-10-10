---
name: "xgk-loadr-gt"
description: "Load a REAL floating-point comparison with relation >."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4154loadrxloadlx.htm"]
---

# LOADR>

Load a REAL floating-point comparison with relation >.

Manual section(s): 4.15.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LOADR>`.

## Editing route

Use `insert_ladder_comparison` after calling `describe_operation` for its current schema.
The writer mnemonic is `R>`; the catalog records opcode `1292` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare S1 > S2 as 32-bit REAL floating-point; the comparison is On when the relation holds and Off otherwise.
- Replace the current bit result (BR) with the comparison result.

## Instruction constraints

- Exact equality of floating-point values requires care because representational precision can change the values.

## Source corrections

- The initial support matrix is a copied GMOV S,D,N table. The actual operand table/body specifies S1,S2 REAL/LREAL comparison; do not use GMOV permissions, counts, step range or flags for this instruction.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 비교하고자 하는 실수 데이터 혹은 실수 데이터를 지정하는 디바이스 번호 | REAL | Unknown | Unknown |
| 2 | S2 | 비교하고자 하는 실수 데이터 혹은 실수 데이터를 지정하는 디바이스 번호 | REAL | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- LOADR X (R=, R<, R>, R<=, R>=, R<>)
- S1 과 S2를 비교하여 X조건과 일치하면 현재의 연산결과를 On합니다.
- S1 과 S2 는 단장형 부동소수점 실수로서 X 조건에 따라 비교연산을 합니다.
- X 조건이 R= 사용에 주의해야 한다. 부동소수점 실수는 정밀도에 따라 값이 달라질 수 있습니다.
- LOADL X (L=, L<, L>, L<=, L>=, L<>)
- S1 과 S2 는 배장형 부동소수점 실수로서 X 조건에 따라 비교연산을 합니다.

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

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LOADR> source figure 1](assets/70ec8d2d6ff5a7f6-NewItem2678.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4154loadrxloadlx.htm` — §4.15.4; source SHA-256 `2e0151647134ff2d3e4b748ea33a576f845bf4cf082cffc8e4fb39062213c39c`.
- Additional behavior/operand evidence: `344.htm`, `4154loadrxloadlx.htm`.

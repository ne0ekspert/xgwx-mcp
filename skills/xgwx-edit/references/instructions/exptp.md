---
name: "xgk-exptp"
description: "Use EXPTP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42714exptexptp.htm"]
---

# EXPTP

Manual section(s): 4.27.14. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `EXPTP`.

Execution form: the source explicitly pairs this command as `(P)`; `EXPTP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `EXPTP`; the catalog records opcode `756` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 지수 연산을 할 진수 입력값 | LREAL | PMK, L, T, C, U, N, D, R | Allowed |
| 2 | S2 | 지수 연산을 할 지수 입력값 | LREAL | PMK, L, T, C, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 디바이스 번호 | LREAL | PMK, L, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- EXPT (지수연산)
- S1로 지정된 영역의 데이터를 S2로 지정된 영역의 데이터로 지수 연산하여 D에 저장합니다.
- 이때, S1, S2와 D의 데이터 타입은 배장형 실수입니다.
- S1의 값이 1.5, S2의 값이 3 일때 연산 결과는 3.375 (1.5( 3))입니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| EXPT(P) | 4~8 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![EXPTP source figure 1](assets/50106e67772714ef-NewItem3300.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42714exptexptp.htm` — §4.27.14; source SHA-256 `535874fdd80e4c56ab38ec67b4f5ed455b51c2c306dcf4048ced0f8c8d32ad05`.
- Additional behavior/operand evidence: `42714exptexptp.htm`, `245.htm`.

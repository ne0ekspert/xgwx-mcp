---
name: "xgk-dincp"
description: "Use DINCP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4161incincpdincdincp1.htm"]
---

# DINCP

Manual section(s): 4.16.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DINCP`.

Execution form: the source explicitly pairs this command as `(P)`; `DINCP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DINCP`; the catalog records opcode `468` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 연산을 수행하게 될 데이터의 주소 | DINT | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- INC (Increment)
- D에 1을 더한 결과를 다시 D에 저장합니다.
- Signed연산을 수행합니다.
- DINC(Double Increment)
- D, D+1의 값에 1을 더한 결과를 다시 D, D+1에 저장합니다.

## Manual flags

- INC/DINC 명령어는 연산 결과로 인한 플래그 처리는 없습니다. 따라서 최대값에서 1증가하는 시점에서의 캐리플래그(F112)가 발생되지 않습니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| INC(P) DINC(P) | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DINCP source figure 1](assets/a4486f3fce618ea4-NewItem2859.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4161incincpdincdincp1.htm` — §4.16.1; source SHA-256 `e2d3575878fcbd9c6d98f334056e8c8218302b375aa6cbcd7305f1653ce6349c`.
- Additional behavior/operand evidence: `4161incincpdincdincp1.htm`, `245.htm`.

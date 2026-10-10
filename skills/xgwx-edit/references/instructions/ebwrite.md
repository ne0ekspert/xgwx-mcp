---
name: "xgk-ebwrite"
description: "Use EBWRITE with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4364ebwrite.htm"]
---

# EBWRITE

Manual section(s): 4.36.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `EBWRITE`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `EBWRITE`; the catalog records opcode `1035` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | Not unambiguously specified in this topic | WORD | PMK, L, Z, U, N, D | Allowed |
| 2 | S2 | Not unambiguously specified in this topic | WORD | PMK, L, Z, U, N, D | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-SU/H/U)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| EBWRITE | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![EBWRITE source figure 1](assets/1cf0970b90858e86-NewItem3448.png)

![EBWRITE source figure 2](assets/6776ffae7b494a3e-NewItem3450.png)

![EBWRITE source figure 3](assets/947423aa468c7f53-NewItem3449.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4364ebwrite.htm` — §4.36.4; source SHA-256 `ee6bba7836df1b29d7ad9cbe81fd6af20df2d291bd9bc7002478e928334be706`.
- Additional behavior/operand evidence: `4364ebwrite.htm`.

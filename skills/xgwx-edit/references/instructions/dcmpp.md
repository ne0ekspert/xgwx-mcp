---
name: "xgk-dcmpp"
description: "Use DCMPP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4141cmpcmppdcmpdcmpp.htm"]
---

# DCMPP

Manual section(s): 4.14.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DCMPP`.

Execution form: the source explicitly pairs this command as `(P)`; `DCMPP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DCMPP`; the catalog records opcode `458` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | Not unambiguously specified in this topic | UDINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | Not unambiguously specified in this topic | UDINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

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
| CMP(P)DCMP(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DCMPP source figure 1](assets/dc8125350ea06e1e-NewItem2780.png)

![DCMPP source figure 2](assets/ce1b23ca0c1d2536-NewItem2782.png)

![DCMPP source figure 3](assets/c6ee9d9ef2a626b0-NewItem2781.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4141cmpcmppdcmpdcmpp.htm` — §4.14.1; source SHA-256 `7b752967f970f055cfab3ecb60d9b6087f4ed00a89ce755aa534feabc5a55416`.
- Additional behavior/operand evidence: `4141cmpcmppdcmpdcmpp.htm`, `245.htm`.

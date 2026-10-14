---
name: "xgk-pidinit"
description: "Reset the settings and state of a selected PID loop."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4288pidinit.htm"]
---

# PIDINIT

Reset the settings and state of a selected PID loop.

Manual section(s): 4.28.8. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `PIDINIT`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `PIDINIT`; the catalog records opcode `1056` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Zero all bit and WORD devices belonging to the designated loop.

## Instruction constraints

- S is a constant loop selector as shown in the instruction/device matrix.

## Source corrections

- The operand table is a copied GWXNR table (S1,S2,D,N), inconsistent with PIDINIT S. Do not supply GWXNR operands or derive the loop range from that table.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | Not unambiguously specified in this topic | WORD | None | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- PIDINIT (PID Initial)
- 해당 PID 루프의 설정 및 상태를 초기화 합니다.
- 이 때에 초기화 되는 영역은 지정된 루프의 모든 비트와 워드 디바이스 이며 0으로 초기화 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| PIDINIT | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![PIDINIT source figure 1](assets/4b3185636ded6bb0-NewItem3333.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4288pidinit.htm` — §4.28.8; source SHA-256 `4d1992c0f54d25ee6a25f388872897bf0c86a1851582faa9eeb7d17fd22dbd6e`.
- Additional behavior/operand evidence: `4288pidinit.htm`.

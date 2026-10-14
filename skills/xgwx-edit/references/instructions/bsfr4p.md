---
name: "xgk-bsfr4p"
description: "Use BSFR4P with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4185bsfr4bsfr4pbsfr8bsfr8p.htm"]
---

# BSFR4P

Manual section(s): 4.18.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BSFR4P`.

Execution form: the source explicitly pairs this command as `(P)`; `BSFR4P` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `BSFR4P`; the catalog records opcode `871` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | BSFR연산의 시작 비트 위치 | NIBBLE/BYTE | PMK, L, D.x, R.x, U | Not allowed |
| 2 | n | D로 지정한 비트 위치부터 4/8비트 중 왼쪽으로 이동시킬 bit수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BSFR4 (Nibble Bit Shift Right)
- D의 4비트 데이터의 각 비트들을 오른쪽으로 n로 지정된 비트수만큼 비트 이동합니다.
- 마지막에 버려진 비트에 따라 캐리 플래그가 셋(Set)/리셋(Reset)됩니다.
- BSFR8 (Byte Bit Shift Right)
- D의 8비트 데이터의 각 비트들을 오른쪽으로 n으로 지정된 비트수만큼 비트 이동합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 캐리 | 마지막으로 버려진 비트에 따라 캐리 플래그가 셋(Set)/리셋(Reset)됩니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSFR4(P) BSFR8(P) | 3~4 | - | - | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BSFR4P source figure 1](assets/38ce002f04561b40-NewItem2914.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4185bsfr4bsfr4pbsfr8bsfr8p.htm` — §4.18.5; source SHA-256 `624e0debdfc4fff5886f1b11e49b2d006b32b9e0c9d1f612718959099f78f6cc`.
- Additional behavior/operand evidence: `4185bsfr4bsfr4pbsfr8bsfr8p.htm`, `245.htm`.

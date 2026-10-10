---
name: "xgk-brlp"
description: "Use BRLP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41810brlbrlp.htm"]
---

# BRLP

Manual section(s): 4.18.10. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BRLP`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BRLP`; the catalog records opcode `1488` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 연산을 수행하게 될 데이터의 주소 | BYTE | Unknown | Unknown |
| 2 | D | 연산결과를 저장할 주소 | BYTE | PMK, F, L, T, C, S, D.x, R.x, U | Not allowed |
| 3 | n1 | 좌측으로 회전시킬 바이트 수 | WORD | Unknown | Unknown |
| 4 | n2 | 좌측으로 회전시킬 횟수 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- BRL (Byte Rotate Left)
- S[0]~S[n1-1]바이트까지의 데이터를 좌측으로 n2번 회전하여 그 결과를 D[0]~D[n1-1]에 저장하는 명령어입니다.
- n1이 0일 경우 연산을 수행하지 않습니다.

## Manual applicability and restrictions

- 적용 기종 : XGK(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BRL (P) | 5~8 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BRLP source figure 1](assets/417b162aa84445c5-NewItem2929.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41810brlbrlp.htm` — §4.18.10; source SHA-256 `9bf1c9a96783f9d927ea30c6e42914a2d613e13001a4b588b8fd393e6725f39a`.
- Additional behavior/operand evidence: `41810brlbrlp.htm`.

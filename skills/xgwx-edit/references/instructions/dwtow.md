---
name: "xgk-dwtow"
description: "Use DWTOW with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4127wtodwwtodwpdwtowdwtowp.htm"]
---

# DWTOW

Manual section(s): 4.12.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DWTOW`.

Execution form: the source explicitly pairs this command as `(P)`; `DWTOW` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DWTOW`; the catalog records opcode `1528` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 변환할 16Bit(WORD)/32Bit(DWORD) 데이터 | WORD/DWORD | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | D | 변환된 32bit(DWORD)/16Bit(WORD)데이터를 저장하는 디바이스 번호 | DWORD/WORD | PMK, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- WTODW, WTODWP(Word to Word)
- S로 지정된 데이터를 부호가 있는 32Bit(DWORD) 데이터로 변환하여 D에 저장하는 명령입니다.
- DWTOW, DWTOWP(DWord to Word)
- S로 지정된 데이터를 부호가 있는 16Bit(WORD) 데이터로 변환하여 D에 저장하는 명령입니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | DWTOW(P)에서 S+1,S로 지정된 데이터 값이 -32768~32767의 범위를 넘어갈 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WTODW(P) DWTOW(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DWTOW source figure 1](assets/0b01913624298120-NewItem2732.png)

![DWTOW source figure 2](assets/8248789604f823a0-NewItem2733.png)

![DWTOW source figure 3](assets/a40b1a4faf3b2638-NewItem2734.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4127wtodwwtodwpdwtowdwtowp.htm` — §4.12.7; source SHA-256 `4e83d06402f845d2130afc178991ab73cf439746e04e9ec36f6f15be2b9cccb5`.
- Additional behavior/operand evidence: `4127wtodwwtodwpdwtowdwtowp.htm`, `245.htm`.

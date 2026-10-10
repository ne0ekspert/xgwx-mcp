---
name: "xgk-bsfl8p"
description: "Use BSFL8P with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4183bsfl4bsfl4pbsfl8bsfl8p.htm"]
---

# BSFL8P

Manual section(s): 4.18.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BSFL8P`.

Execution form: the source explicitly pairs this command as `(P)`; `BSFL8P` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BSFL8P`; the catalog records opcode `869` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | BSFL연산의 시작 비트 위치 | NIBBLE/BYTE | PMK, L, D.x, R.x, U | Not allowed |
| 2 | n | D로 지정한 비트 위치부터 4/8비트 중 왼쪽으로 이동시킬 bit수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BSFL4 (Nibble Bit Shift Left)
- D로 지정한 비트 위치부터 4비트 중 n만큼 왼쪽으로 이동시킵니다.
- BSFL8 (Byte Bit Shift Left)
- D의 8비트 데이터의 각 비트들을 왼쪽으로 n으로 지정된 비트수만큼 비트이동합니다.
- 마지막에 버려진 비트에 따라 캐리플래그를 On/Off 합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 캐리 | 마지막으로 버려진 비트에 따라 캐리 플래그를 On/Off 합니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSFL4(P) BSFL8(P) | 3~5 | - | - | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BSFL8P source figure 1](assets/290864d62adc8eed-NewItem2909.png)

![BSFL8P source figure 2](assets/3bdf1b3fa65568ee-NewItem2910.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4183bsfl4bsfl4pbsfl8bsfl8p.htm` — §4.18.3; source SHA-256 `9236d9a896a80b080e3308b9042bf032f91292df803657e96f0cff616070ee62`.
- Additional behavior/operand evidence: `4183bsfl4bsfl4pbsfl8bsfl8p.htm`, `245.htm`.

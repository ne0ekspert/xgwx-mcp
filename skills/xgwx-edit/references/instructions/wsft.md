---
name: "xgk-wsft"
description: "Use WSFT with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4186wsftwsftp.htm"]
---

# WSFT

Manual section(s): 4.18.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `WSFT`.

Execution form: the source explicitly pairs this command as `(P)`; `WSFT` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `WSFT`; the catalog records opcode `204` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | St | WSFT 연산의 시작 워드 데이터의 주소 | WORD | Unknown | Unknown |
| 2 | Ed | WSFT 연산의 끝 워드 데이터의 주소 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- WSFT (Word Shift)
- 시작 워드(St)에서 끝 워드(Ed) 방향으로 1워드 단위로 shift 합니다.
- 워드 Shift 방향
- S1 < S2 (예 : WSFT D0000 D0003) ® 아래 방향
- S1 > S2 (예 : WSFT D0003 D0000) ® 위 방향

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WSFT(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![WSFT source figure 1](assets/97fe78dce2cde2e5-NewItem2916.png)

![WSFT source figure 2](assets/7072b33979028df0-NewItem2917.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4186wsftwsftp.htm` — §4.18.6; source SHA-256 `0234016ada3c0d6e695d87a048583bd3b947ed299dda7ff155830955d622972f`.
- Additional behavior/operand evidence: `4186wsftwsftp.htm`, `245.htm`.

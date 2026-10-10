---
name: "xgk-dbinp"
description: "Use DBINP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4123binbinpdbindbinp.htm"]
---

# DBINP

Manual section(s): 4.12.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DBINP`.

Execution form: the source explicitly pairs this command as `(P)`; `DBINP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DBINP`; the catalog records opcode `430` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | BCD데이터가 저장되어진 영역번호 또는 BCD데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | BIN로 변환된 데이터를 저장하게 될 영역 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BIN(Binary)
- S로 지정된 디바이스의 BCD데이터(0~9999)를 BIN데이터로 변환하여 D에 저장합니다.
- DBIN(Double Binary)
- S+1,S로 지정된 디바이스의 BCD데이터(h0~h99999999)를 BIN데이터로 변환하여 D+1,D에 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | BIN(P)의 경우, S의 데이터가 BCD형태(h0~h9999)가 아닐 경우. DBIN(P)의 경우, S+1,S의 데이터가 BCD형태(h0~h99999999)가 아닐 경우. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BIN(P) DBIN(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DBINP source figure 1](assets/24bf1094f0ca4f97-NewItem2718.png)

![DBINP source figure 2](assets/24d0375edb93c2d2-NewItem2719.png)

![DBINP source figure 3](assets/f3f1fd624b7a0310-NewItem2720.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4123binbinpdbindbinp.htm` — §4.12.3; source SHA-256 `c2d04325ac80e7965f081638bffc5e0a64271a69b6e987e9b8b0f6b00772aada`.
- Additional behavior/operand evidence: `4123binbinpdbindbinp.htm`, `245.htm`.

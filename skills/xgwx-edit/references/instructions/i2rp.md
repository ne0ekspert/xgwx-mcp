---
name: "xgk-i2rp"
description: "Use I2RP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4131i2ri2rpi2li2lp.htm"]
---

# I2RP

Manual section(s): 4.13.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `I2RP`.

Execution form: the source explicitly pairs this command as `(P)`; `I2RP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `I2RP`; the catalog records opcode `436` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 데이터가 저장되어 있는 디바이스 번호 | INT | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | DestinatiOn영역의 디바이스 번호 | REAL | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- I2R(Integer to Real)
- S로 지정된 16비트 정수형 데이터를 단장형 실수(32비트)로 변환하여 D+1, D에 저장합니다.
- I2L(Integer to Long Real)
- S로 지정된 16비트 정수형 데이터를 배장형 실수(64비트)로 변환하여 D+3,D+2,D+1,D에 저장합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| I2R(P) I2L(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![I2RP source figure 1](assets/dfc5ca9a7a73a9bc-NewItem2735.png)

![I2RP source figure 2](assets/cd01f14e207cde5e-NewItem2736.png)

![I2RP source figure 3](assets/985af8d959cd3f02-NewItem2737.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4131i2ri2rpi2li2lp.htm` — §4.13.1; source SHA-256 `5592d4ca32f015591b7beb15b718f927224caf1015f4af2f58970d0a94721a1d`.
- Additional behavior/operand evidence: `4131i2ri2rpi2li2lp.htm`, `245.htm`.

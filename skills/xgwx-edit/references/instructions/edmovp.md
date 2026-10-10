---
name: "xgk-edmovp"
description: "Use EDMOVP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4362emovemovpedmovedmovp.htm"]
---

# EDMOVP

Manual section(s): 4.36.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `EDMOVP`.

Execution form: the source explicitly pairs this command as `(P)`; `EDMOVP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `EDMOVP`; the catalog records opcode `811` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 플래시 영역의 블록번호 | WORD | PMK, L, Z, U, N, D | Allowed |
| 2 | S2 | S1으로 지정한 블록내에서 원하는 데이터가 들어있는 디바이스 번호 | WORD | PMK, L, Z, U, N, D | Allowed |
| 3 | D | 저장할 디바이스의 번호 | WORD | PMK, L, Z, U, N, D | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- EMOV (플래시 메모리 워드 데이터 전송)
- S1으로 지정한 블록내의 S2의 워드 데이터를 D로 전송합니다.
- EDMOV (플래시 메모리 더블워드 데이터 전송)
- S1으로 지정한 블록내의 S2+1, S2의 더블 워드 데이터를 D+1, D로 전송합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-SU/H/U)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| EMOV(P) EDMOV(P) | 4~7 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![EDMOVP source figure 1](assets/c6aacd1ae568980e-NewItem3441.png)

![EDMOVP source figure 2](assets/ff201eac3a2c918a-NewItem3442.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4362emovemovpedmovedmovp.htm` — §4.36.2; source SHA-256 `0dbaf8022e475874811834619f0f78358985c28b02a0cce5d11bdaf4d4dc2d22`.
- Additional behavior/operand evidence: `4362emovemovpedmovedmovp.htm`, `245.htm`.

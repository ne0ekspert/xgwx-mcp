---
name: "xgk-bsftp"
description: "Use BSFTP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4181bsftbsftp.htm"]
---

# BSFTP

Manual section(s): 4.18.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BSFTP`.

Execution form: the source explicitly pairs this command as `(P)`; `BSFTP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BSFTP`; the catalog records opcode `498` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | St | BSFT 연산의 시작 비트 | WORD/DWORD | PMK, L, D.x, R.x | Not allowed |
| 2 | Ed | BSFT 연산의 끝 비트 | WORD | PMK, L, D.x, R.x | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- BSFT (Bit Shift)
- 시작 비트(St)로부터 끝 비트(Ed) 방향으로 비트 데이터를 각각 1비트씩 shift합니다.
- 비트 shift 방향:
- (1) St < Ed : left shift
- (2) St > Ed : right shift
- DROL (Double Rotate Left)
- D 과 D+1 의 32비트 데이터를 좌측으로 캐리 플래그를 포함하지 않고 n bit 회전합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSFT(P) | 3~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BSFTP source figure 1](assets/3b3d7a17e6e5544e-NewItem2902.png)

![BSFTP source figure 2](assets/b0e0b4da82d3f806-NewItem2903.png)

![BSFTP source figure 3](assets/6110d71a1ac6f5ab-NewItem2880.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4181bsftbsftp.htm` — §4.18.1; source SHA-256 `004bbb2a36036b0447609f7ab647120c6aec8f17a0b94dff6135bdfed41bd085`.
- Additional behavior/operand evidence: `4181bsftbsftp.htm`, `245.htm`.

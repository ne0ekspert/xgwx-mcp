---
name: "xgk-len"
description: "Use LEN with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4267lenlenp.htm"]
---

# LEN

Manual section(s): 4.26.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LEN`.

Execution form: the source explicitly pairs this command as `(P)`; `LEN` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `LEN`; the catalog records opcode `299` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 문자열의 시작위치 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | D | 문자열 길이를 저장할 위치 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- LEN (Length)
- S로부터 한 워드당 2개씩 ASCII로 저장된 문자열의 길이를 계산하여 D에 저장합니다.
- S로 지정된 문자열이31글자를 넘어서도 NULL 코드가 없으면 31글자를 리턴하고 에러는 없습니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| LEN(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LEN source figure 1](assets/370b1d5e2756ef77-NewItem3204.png)

![LEN source figure 2](assets/aa86baacc1f8e6cb-NewItem3205.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4267lenlenp.htm` — §4.26.7; source SHA-256 `cf82d07754f8177f1969bcec66fc33d7b4f7c7bbeca45c2a4fec615e2b29cd46`.
- Additional behavior/operand evidence: `4267lenlenp.htm`, `245.htm`.

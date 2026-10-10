---
name: "xgk-bsump"
description: "Use BSUMP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4241bsumbsumpdbsumdbsump.htm"]
---

# BSUMP

Manual section(s): 4.24.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BSUMP`.

Execution form: the source explicitly pairs this command as `(P)`; `BSUMP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BSUMP`; the catalog records opcode `620` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 1의 개수를 카운트하게 되는 워드 데이터의 주소 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 카운트한 결과를 저장할 주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BSUM (Bit Summary)
- S로 지정된 영역의 워드 데이터 중 값이 1인 비트의 개수를 세어서 그 결과를 D에 16진수로 저장합니다.
- 연산결과가 0일 때 제로 플래그를 셋(Set)합니다.
- DBSUM (Double Bit Summary)
- S로 지정된 영역의 더블 워드 데이터 중 값이 1인 비트의 개수를 세어서 그 결과를 D에 16진수로 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 제로 | 연산결과가 제로이면 셋(Set)됩니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSUM(P) DBSUM(P) | 2~4 | - | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BSUMP source figure 1](assets/87a7af1df3bf11b9-NewItem3081.png)

![BSUMP source figure 2](assets/a46c4bb72175f175-NewItem3082.png)

![BSUMP source figure 3](assets/33d08c0ca0fa88b5-NewItem3083.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4241bsumbsumpdbsumdbsump.htm` — §4.24.1; source SHA-256 `fca7528c7756346a1e00bd040558770dd0bd973b7359c1d7752758f6cf55d72b`.
- Additional behavior/operand evidence: `4241bsumbsumpdbsumdbsump.htm`, `245.htm`.

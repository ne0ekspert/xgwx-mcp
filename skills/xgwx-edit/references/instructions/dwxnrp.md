---
name: "xgk-dwxnrp"
description: "Use DWXNRP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4224wxnrwxnrpdwxnrdwxnrp1.htm"]
---

# DWXNRP

Manual section(s): 4.22.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DWXNRP`.

Execution form: the source explicitly pairs this command as `(P)`; `DWXNRP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DWXNRP`; the catalog records opcode `608` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 (D)WXNR연산을 하게 되는 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 (D)WXNR 연산을 하게 되는 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- WXNR( Word Exclusive NOR )
- 워드데이터 S1과 S2의 각 비트를 부정 배타적 논리합(WXNR)하여 그 결과를 D에 저장합니다.
- 부정 배타적 논리합(WXNR)은 S1, S2 각각의 비트값이 다르면 ‘0’이고 같으면 ’1’로 D의 해당 비트에 저장하는 연산을 말합니다.
- DWXNR( Double Word Exclusive NOR )
- 더블 워드 데이터 (S1+1,S1)과 (S2+1,S2)의 각 비트를 부정 배타적 논리합(DWXNR)하여 그 결과를 (D+1,D)의 더블워드 영역에 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 제로 | 연산결과가 제로이면 셋(Set)됩니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WXNR(P) DWXNR(P) | 4~6 | - | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DWXNRP source figure 1](assets/c369754e1b6147d5-NewItem3028.png)

![DWXNRP source figure 2](assets/d3c3cc1c27cda379-NewItem3029.png)

![DWXNRP source figure 3](assets/8202c91552b8d930-NewItem3030.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4224wxnrwxnrpdwxnrdwxnrp1.htm` — §4.22.4; source SHA-256 `b3b9f66cc2641e6aae774b6fbf3e3855c145fd1899c6ad4868666755cd37172d`.
- Additional behavior/operand evidence: `4224wxnrwxnrpdwxnrdwxnrp1.htm`, `245.htm`.

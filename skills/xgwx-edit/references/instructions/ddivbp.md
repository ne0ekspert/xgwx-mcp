---
name: "xgk-ddivbp"
description: "Use DDIVBP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4214divbdivbpddivbddivbp.htm"]
---

# DDIVBP

Manual section(s): 4.21.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DDIVBP`.

Execution form: the source explicitly pairs this command as `(P)`; `DDIVBP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DDIVBP`; the catalog records opcode `592` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 BCD 나눗셈을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 BCD 나눗셈을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD/DWORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DIVB (BCD Divide)
- BCD데이터 S1을 S2로 나눈 몫을 D에 저장합니다.
- BCD 데이터 S1을 S2로 나눈 나머지를 D다음 워드에 저장합니다.
- 연산결과에 따라 에러(F110), 제로(F111) 플래그를 셋(Set)합니다.
- DDIVB (BCD Double Divide)
- BCD 데이터 (S1+1, S1)을 (S2+1, S2)로 나눈 몫을 (D+1, D)에 저장합니다.
- BCD 데이터 (S1+1, S1)을 (S2+1, S2)로 나눈 나머지를 (D+3, D+2)에 저장합니다.
- 연산결과에 따라 에러(F110), 제로(F111) 플래그를 셋(SET)합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S1과 S2의 값이 BCD형식이 아닐 경우, S2의 값이 0일 경우 | F110 |
| 제로 | 연산결과가 제로이면 셋(Set)합니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DIVB(P) DDIVB(P) | 4~6 | O | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DDIVBP source figure 1](assets/698d1a482a9a0f43-NewItem3009.png)

![DDIVBP source figure 2](assets/8c525d4933a3856f-NewItem3010.png)

![DDIVBP source figure 3](assets/e10a54c48ee4fb44-NewItem3011.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4214divbdivbpddivbddivbp.htm` — §4.21.4; source SHA-256 `db331a7e69551df85767206c0b9870cfa08f526a5bed7a8e449876ead3312977`.
- Additional behavior/operand evidence: `4214divbdivbpddivbddivbp.htm`, `245.htm`.

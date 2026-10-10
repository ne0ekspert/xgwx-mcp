---
name: "xgk-mulu"
description: "Multiply two unsigned 16-bit integers."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4207mulumulupdmuludmulup.htm"]
---

# MULU

Multiply two unsigned 16-bit integers.

Manual section(s): 4.20.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `MULU`.

Execution form: the source explicitly pairs this command as `(P)`; `MULU` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `MULU`; the catalog records opcode `224` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- S1 and S2 are 16-bit operands. Store the full 32-bit product in consecutive WORDs beginning at D.

## Instruction constraints

- Reserve 2 destination WORDs. A zero result sets F111.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 곱셈연산을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 곱셈연산을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | DWORD/LWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- MULU (Unsigned Binary Multiply)
- 워드데이터 S1과 S2를 곱한 후 결과를 (D+1,D)(32bit)에 저장합니다.
- 이 때 Unsigned 연산을 실행합니다.
- DMULU (Unsigned Binary Double Multiply)
- 더블 워드데이터 (S1+1,S1)과 (S2+1,S2)를 곱한 후 결과를 (D+3,D+2,D+1,D)(64bit)에 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 제로 | 연산결과가 제로이면 셋(Set)합니다 | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| MULU(P) DMULU(P) | 4~6 | - | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![MULU source figure 1](assets/e994833eafdace35-NewItem2974.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4207mulumulupdmuludmulup.htm` — §4.20.7; source SHA-256 `c72f3d3b0cdbdb64c8d951d348f4908e246bcbb86f856bdecee1534fbd9001a0`.
- Additional behavior/operand evidence: `4207mulumulupdmuludmulup.htm`, `245.htm`.

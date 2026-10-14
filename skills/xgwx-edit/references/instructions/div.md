---
name: "xgk-div"
description: "Divide two signed 16-bit integers and retain the remainder."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4204divdivpddivddivp.htm"]
---

# DIV

Divide two signed 16-bit integers and retain the remainder.

Manual section(s): 4.20.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DIV`.

Execution form: the source explicitly pairs this command as `(P)`; `DIV` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DIV`; the catalog records opcode `218` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- For 16-bit operands, the quotient is stored in D and the remainder in D+1.

## Instruction constraints

- A zero divisor S2 sets F110.
- Reserve 2 consecutive destination WORDs.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 나눗셈연산을 실행할 데이터 | INT/DINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 나눗셈연산을 실행할 데이터 | INT/DINT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | INT/DINT | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DIV (Signed Binary Divide)
- 워드 데이터 S1을 S2로 나눈 후 몫을 D(16bit)에 나머지를 D+1에 저장합니다.
- 이 때 signed연산을 실행합니다.
- DDIV (Signed Binary Double Divide)
- (S1+1,S1)을 (S2+1,S2)로 나눈 후 몫을 (D+1,D)에, 나머지를 (D+3,D+2)에 저장합니다.
- 이 때 Signed 연산을 실행합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S2의 값이 0일 때 셋(Set)합니다. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DIV(P) DDIV(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DIV source figure 1](assets/6651106d1aa0b0a1-NewItem2966.png)

![DIV source figure 2](assets/7e9bf4c7ff354722-NewItem2967.png)

![DIV source figure 3](assets/1eac9bb2fe758a1d-NewItem2968.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4204divdivpddivddivp.htm` — §4.20.4; source SHA-256 `20746968d489cffe04c0bc1a68631d5c3bbdea892a7fa8b24dbf432b48940355`.
- Additional behavior/operand evidence: `4204divdivpddivddivp.htm`, `245.htm`.

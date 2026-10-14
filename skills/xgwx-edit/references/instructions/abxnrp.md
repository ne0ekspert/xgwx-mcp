---
name: "xgk-abxnrp"
description: "Use ABXNRP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42216abxnrabxnrp1.htm"]
---

# ABXNRP

Manual section(s): 4.22.16. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ABXNRP`.

Execution form: the source explicitly pairs this command as `(P)`; `ABXNRP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `ABXNRP`; the catalog records opcode `1486` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | ABXOR연산을 시작하게 되는 데이터 주소 | BYTE | PMK, L, D.x, R.x, U | Not allowed |
| 2 | D | ABXOR연산의 결과를 저장할 주소 | BYTE | PMK, L, D.x, R.x, U | Not allowed |
| 3 | N | 바이트데이터끼리의 XOR연산을 하는 개수 | WORD | PMK, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- ABXOR (Arrary Byte XOR)
- 바이트데이터(8비트) S[0]부터 S[n-1]까지의 바이트를 서로XNR하여 결과를 D에 저장합니다.
- N 값이 1일 때 s[0]의 값을 복사하여 D에 저장합니다. S[0]의 값이 0이어도 제로 플래그가 셋(Set)되지 않습니다.
- N 값이 0일 연산을 수행하지 않습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 값이 지정 영역을 벗어날 때 셋(Set)됩니다. | F110 |
| 제로 | 연산결과가 제로이면 셋(Set)합니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ABXNR(P) | 5~7 | O | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ABXNRP source figure 1](assets/150c8ee5023a894c-NewItem3073.png)

![ABXNRP source figure 2](assets/a41b53a166b91477-NewItem3074.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42216abxnrabxnrp1.htm` — §4.22.16; source SHA-256 `9c6555ad0f820bb4b68f94a765eb1fac1f81516ef25fbdbf7fd730cdf2c1537c`.
- Additional behavior/operand evidence: `42216abxnrabxnrp1.htm`, `245.htm`.

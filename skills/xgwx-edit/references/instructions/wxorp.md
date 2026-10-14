---
name: "xgk-wxorp"
description: "Use WXORP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4223wxorwxorpdwxordwxorp1.htm"]
---

# WXORP

Manual section(s): 4.22.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `WXORP`.

Execution form: the source explicitly pairs this command as `(P)`; `WXORP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `WXORP`; the catalog records opcode `602` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 (D)WXOR연산을 하게 되는 데이터 | WORD/DWORD | PMKL, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 (D)WXOR 연산을 하게 되는 데이터 | WORD/DWORD | PMKL, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD/DWORD | PMKL, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- WXOR( Word Exclusive OR )
- 워드데이터 S1과 S2의 각 비트를 배타적 논리합(XOR)하여 그 결과를 D에 저장합니다.
- 배타적 논리합(WXOR)은 S1, S2 각각의 비트값이 다르면 ‘1’이고 같으면 ’0’으로 D의 해당 비트에 저장하는 연산을 말합니다.
- DWXOR( Double Word Exclusive OR )
- 더블 워드 데이터 (S1+1,S1)과 (S2+1,S2)의 각 비트를 배타적 논리합(XOR)하여 그 결과를 (D+1,D)에 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 제로 | 연산결과가 제로이면 셋(Set)됩니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WXOR(P) DWXOR(P) | 4~6 | - | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![WXORP source figure 1](assets/947157e373396c42-NewItem3024.png)

![WXORP source figure 2](assets/d6538d7b5a1e3802-NewItem3025.png)

![WXORP source figure 3](assets/80e41e9c8d1dc874-NewItem3026.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4223wxorwxorpdwxordwxorp1.htm` — §4.22.3; source SHA-256 `1646681095edcf565ee7eb469890cbdf984d454982aa9d2f9561a440a910a450`.
- Additional behavior/operand evidence: `4223wxorwxorpdwxordwxorp1.htm`, `245.htm`.

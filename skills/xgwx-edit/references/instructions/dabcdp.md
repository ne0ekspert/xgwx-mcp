---
name: "xgk-dabcdp"
description: "Use DABCDP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4266dabcddabcdpddabcdddabcdp.htm"]
---

# DABCDP

Manual section(s): 4.26.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DABCDP`.

Execution form: the source explicitly pairs this command as `(P)`; `DABCDP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DABCDP`; the catalog records opcode `692` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | BCD값으로 변환할 10진 ASCII 데이터가 저장되어 있는 주소 | STRING | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D | 변환값을 저장할 위치 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DABCD (Decimal ASCII to BCD)
- ASCII로 저장된 10진수값을 BCD값으로 변환하여 D에 저장합니다.
- 데이터 D는 Unsigned로 저장됩니다.
- DABCD에서 연산 범위는 h0000부터 h9999까지입니다.
- DDABCD (Double Decimal ASCII to BCD)
- DDABCD에서 연산 범위는 h00000000부터 h99999999까지입니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ASCII 문자열에 ‘0’~’9’ 이외의 문자가 존재할 경우 셋(Set)합니다. ASCII 문자열의 길이가 4(DABCD)/8(DDABCD) 를 넘을 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DABCD(P) DDABCD(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DABCDP source figure 1](assets/a0ba55d8f6fbe63f-NewItem3198.png)

![DABCDP source figure 2](assets/274aa021446231b8-NewItem3200.png)

![DABCDP source figure 3](assets/a96c417a609bea5f-NewItem3199.png)

![DABCDP source figure 4](assets/e8347bba30f3945e-NewItem3201.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4266dabcddabcdpddabcdddabcdp.htm` — §4.26.6; source SHA-256 `a4c35f1f3265a96bb8228cdc0bf827e346a9d41893776785f72c7fe8d53b8699`.
- Additional behavior/operand evidence: `4266dabcddabcdpddabcdddabcdp.htm`, `245.htm`.

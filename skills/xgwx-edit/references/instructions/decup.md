---
name: "xgk-decup"
description: "Use DECUP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4166decudecupddecuddecup1.htm"]
---

# DECUP

Manual section(s): 4.16.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DECUP`.

Execution form: the source explicitly pairs this command as `(P)`; `DECUP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DECUP`; the catalog records opcode `478` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 연산을 수행하게 될 데이터의 주소 | UINT | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DECU (Decrement)
- D에 1을 뺀 결과를 다시 D에 저장합니다.
- D는 Unsigned값으로 처리됩니다.
- D가 0 (h0000)일 때 DECU(P)가 실행되면 65,535(hFFFF)을 출력하며 캐리 플래그를 셋(Set)합니다.
- D가 1일 때 (D)DECU(P)가 실행되면 0을 출력하며 제로 플래그를 셋(Set)합니다.
- DDECU (Double Decrement)
- D, D+1에 1을 뺀 결과를 다시 D, D+1에 저장합니다.
- D, D+1은 Unsigned값으로 처리됩니다.
- D, D+1이 1일 때 (D)DECU(P)가 실행되면 0을 출력하며 제로 플래그를 셋(Set)합니다.
- D, D+1이 0 (h00000000)일 때 DDECU(P)가 실행되면 4,294,967,295(hFFFFFFFF)을 출력하며 캐리 플래그를 셋(Set)합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 제로 | D가 1일 때 (D)DEC(P)가 실행되면 셋(Set)합니다. | F111 |
| 캐리 | D가 -1(FFFFD가 0 에서 hFFFF일 때 셋(SET) | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DECU(P) DDECU(P) | 2~3 | - | O | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DECUP source figure 1](assets/24eb603e8ea8ef26-NewItem2876.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4166decudecupddecuddecup1.htm` — §4.16.6; source SHA-256 `019a8b45d01bef9498b945e1b27e8d62295af4155416bbd31bed0e691f200e4b`.
- Additional behavior/operand evidence: `4166decudecupddecuddecup1.htm`, `245.htm`.

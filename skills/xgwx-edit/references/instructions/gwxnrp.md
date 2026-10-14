---
name: "xgk-gwxnrp"
description: "Use GWXNRP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4228gwxnrgwxnrp1.htm"]
---

# GWXNRP

Manual section(s): 4.22.8. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GWXNRP`.

Execution form: the source explicitly pairs this command as `(P)`; `GWXNRP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `GWXNRP`; the catalog records opcode `616` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 GWXNR연산을 시작하게 되는 데이터의 주소 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | S2 | S1과 GWXNR연산을 시작하게 되는 데이터의 주소 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 4 | N | WXNR연산을 하는 개수 | WORD | PMK, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- GWXNR (Group Word XNR)
- S1과 S2에서 부터 워드단위로 WXNR를 N번 실행하여 그 결과들을 D에 차례로 저장합니다.
- S2에는 h0000 ~ hFFFF(BIN 16비트)의 정수를 지정할 수 있습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 값이 지정 영역을 벗어날 때 셋(Set)됩니다. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GWXNR(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GWXNRP source figure 1](assets/1874dff625558f86-NewItem3044.png)

![GWXNRP source figure 2](assets/ce1cc0ed240fcfb4-NewItem3046.png)

![GWXNRP source figure 3](assets/87bc00a5ad30bc4e-NewItem3045.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4228gwxnrgwxnrp1.htm` — §4.22.8; source SHA-256 `3f03eb0b4f2ba11b3817d31597c4511f458f034921b6853042552dfc1567c9ac`.
- Additional behavior/operand evidence: `4228gwxnrgwxnrp1.htm`, `245.htm`.

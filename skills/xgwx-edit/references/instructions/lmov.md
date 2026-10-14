---
name: "xgk-lmov"
description: "Use LMOV with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4118rmovrmovplmovlmovp2.htm"]
---

# LMOV

Manual section(s): 4.11.8. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LMOV`.

Execution form: the source explicitly pairs this command as `(P)`; `LMOV` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `LMOV`; the catalog records opcode `150` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전송하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호 | LREAL | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 전송된 데이터를 저장할 디바이스 번호 | LREAL | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- RMOV(Real Move)
- S+1,S로 지정된 디바이스의 실수(Real) 데이터를 D+1,D에 전송합니다.
- S에 상수를 입력할 경우, 10진수 형식으로만 입력 가능 합니다. 16진수 입력은 허용하지 않습니다.
- LMOV(Long Real Move)
- S+3,S+2,S+1,S의 Long Real 데이터를 D+3,D+2,D+1,D로 전송합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| RMOV(P) LMOV(P) | 2~5 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LMOV source figure 1](assets/841479333d38b4b8-NewItem2695.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4118rmovrmovplmovlmovp2.htm` — §4.11.8; source SHA-256 `63e46594030dff61ac19fd25615b3c6820aa297ae084ac2e17a3d19c0961f87d`.
- Additional behavior/operand evidence: `4118rmovrmovplmovlmovp2.htm`, `245.htm`.

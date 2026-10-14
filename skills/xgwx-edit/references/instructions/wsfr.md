---
name: "xgk-wsfr"
description: "Use WSFR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4187wsflwsflpwsfrwsfrp.htm"]
---

# WSFR

Manual section(s): 4.18.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `WSFR`.

Execution form: the source explicitly pairs this command as `(P)`; `WSFR` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `WSFR`; the catalog records opcode `206` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D1 | 워드 이동을 할 구간의 디바이스 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D2 | 워드 이동을 할 구간의 디바이스 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | n | 1회 수행 시 이동시킬 워드의 개수 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- WSFL (Word Shift Left)
- D1과 D2사이의 워드를 N번 상위 워드 방향으로 이동시킵니다. 이때 이동시킨 개수만큼의 하위 워드에는 0이 저장됩니다.
- WSFR (Word Shift Right)
- D1과 D2사이의 워드를 N번 하위 워드 방향으로 이동시킵니다. 이때 이동시킨 개수만큼의 상위 워드에는 0이 저장됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WSFL(P) WSFR(P) | 4~6 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![WSFR source figure 1](assets/e9214b3178b45b43-NewItem2920.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4187wsflwsflpwsfrwsfrp.htm` — §4.18.7; source SHA-256 `d807a1e3a5f6ce19650a6a453dd3ebe19aa386d101da93802fe33edf33a48e61`.
- Additional behavior/operand evidence: `4187wsflwsflpwsfrwsfrp.htm`, `245.htm`.

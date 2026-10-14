---
name: "xgk-rol4"
description: "Use ROL4 with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4172rol4rol4prol8rol8p.htm"]
---

# ROL4

Manual section(s): 4.17.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ROL4`.

Execution form: the source explicitly pairs this command as `(P)`; `ROL4` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `ROL4`; the catalog records opcode `388` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 연산을 수행하게 될 데이터의 주소 | NIBBLE/BYTE | PMK, L, D.x, R.x, U | Not allowed |
| 2 | n | 좌측으로 회전시킬 비트수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- ROL4(Rotate Left)
- D의 4개 비트를 n의 지정된 비트수 만큼 좌측으로 비트 회전하며 4개의 비트중 최상위 비트는 캐리 플래그와 최하위 비트로 회전 (4비트내에서 회전)
- 회전중 캐리가 발생하면 캐리 플래그를 셋(SET)
- ROL8 (Double Rotate Left)
- D의 8개 비트를 n의 지정된 비트수 좌측으로 회전하며 8개의 비트중 최상위 비트는 캐리 플래그와 최하위 비트로 회전(8비트내에서 회전)

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 캐리 | 회전중 캐리가 발생하면 캐리 플래그를 셋(Set)합니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ROL4(P) ROL8(P) | 2~4 | - | - | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ROL4 source figure 1](assets/29784c0d9f789b7f-NewItem2882.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4172rol4rol4prol8rol8p.htm` — §4.17.2; source SHA-256 `fc0b6872dc08cdc8d818de660fe30eee140f18fb62b66d15fa790544a1b3c24c`.
- Additional behavior/operand evidence: `4172rol4rol4prol8rol8p.htm`, `245.htm`.

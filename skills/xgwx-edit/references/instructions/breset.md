---
name: "xgk-breset"
description: "Use BRESET with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4385bsetbreset.htm"]
---

# BRESET

Manual section(s): 4.38.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BRESET`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `BRESET`; the catalog records opcode `1073` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 해당 디바이스의 워드 영역 | WORD | Unknown | Unknown |
| 2 | n | 워드 영역의 n번째 비트 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- BSET
- 조건 만족시 D로 지정된 영역의 n번째 비트를 셋(Set)합니다.
- n값의 하위 4비트만을 취해서 비트 위치를 결정합니다. 따라서 n값이 워드크기를 벗어날 경우 에러가 발생하지 않습니다.
- BRESET
- 조건 만족시 D로 지정된 영역의 n번째 비트를 리셋(Reset)합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSET BRESET | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BRESET source figure 1](assets/af573c824b2efe61-NewItem3473.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4385bsetbreset.htm` — §4.38.5; source SHA-256 `e93104aa0ac47b81286e69ab677d3532f3177f56cdde7a70327623f548789f78`.
- Additional behavior/operand evidence: `4385bsetbreset.htm`.

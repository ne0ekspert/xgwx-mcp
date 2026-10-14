---
name: "xgk-ctd"
description: "Count rising inputs downward from a preset."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4102ctd.htm"]
---

# CTD

Count rising inputs downward from a preset.

Manual section(s): 4.10.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `CTD`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `CTD`; the catalog records opcode `923` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Each rising pulse decrements the current value; zero turns the contact On.
- RST clears the contact and restores the current value to N.

## Instruction constraints

- C selects a counter device; N is a WORD preset in 0..65,535.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | C | 사용하고자 하는 카운터 접점 | WORD | C | Not allowed |
| 2 | N | 설정치 (0 ~ 65,535) | WORD | PMK, U, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 입상 펄스가 입력될 때마다 설정치로부터 1씩 감산을 하여 0이 되면 출력을 On합니다.
- 리셋(Reset) 신호가 On하면 출력을 Off시키며 현재치는 설정치가 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| CTD | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![CTD source figure 1](assets/3acb2ff492cecd3d-NewItem2634.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4102ctd.htm` — §4.10.2; source SHA-256 `d2d3686c4ea570e66e0795adcc65a953aedb18c4a6d790fa1769d0303acde37f`.
- Additional behavior/operand evidence: `4102ctd.htm`.

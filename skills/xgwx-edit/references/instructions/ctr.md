---
name: "xgk-ctr"
description: "Use CTR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4105ctr1.htm"]
---

# CTR

Manual section(s): 4.10.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `CTR`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `CTR`; the catalog records opcode `929` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | C | 사용하고자 하는 카운터 접점 | WORD | C | Not allowed |
| 2 | N | 설정치 ( 0 ~ 65,535 ) | WORD | PMK, U, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 입상 펄스가 입력될 때마다 현재치를 +1하고 현재치가 설정치에 도달한 후 입력신호가 Off®On되면 현재치는 0으로 됩니다.
- 현재치가 설정치에 도달하면 출력은 On됩니다.
- 현재치가 설정치 미만이거나 리셋(Reset) 조건이 On이면 출력은 Off됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| CTR | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![CTR source figure 1](assets/b3ea2f3fee4b772e-NewItem2652.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4105ctr1.htm` — §4.10.5; source SHA-256 `ffc9f43999bf73f5566d8169f8b19d66ab638079f236099aaeb6c49fbb52e47a`.
- Additional behavior/operand evidence: `4105ctr1.htm`.

---
name: "xgk-ctu"
description: "Count rising inputs upward."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4103ctu.htm"]
---

# CTU

Count rising inputs upward.

Manual section(s): 4.10.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `CTU`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `CTU`; the catalog records opcode `925` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Each rising pulse increments the current value by one. The contact is On when the value reaches or exceeds N; counting continues up to 65,535.
- RST clears the contact and current value.

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

- 입상 펄스가 입력될 때마다 현재치를 +1하고 현재치가 설정치 이상이면 출력을 On하고 카운터 최대치(65,535)까지 Count합니다.
- 리셋(Reset) 신호가 On하면 출력을 Off시키며 현재치는 0이 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| CTU | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![CTU source figure 1](assets/f162563482c4f90b-NewItem2639.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4103ctu.htm` — §4.10.3; source SHA-256 `da1bbf7dc303bded8b5b83c740e054a36cb97bbe94fc37aea0aa1aa309ae5438`.
- Additional behavior/operand evidence: `4103ctu.htm`.

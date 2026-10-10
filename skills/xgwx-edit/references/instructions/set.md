---
name: "xgk-set"
description: "Use SET with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["452set.htm"]
---

# SET

Manual section(s): 4.5.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SET`.

## Editing route

Use `edit_ladder_cell` after calling `describe_operation` for its current schema.
Set the replacement element kind to `Set`. `SET` describes `output_column` placement; it is represented by a physical cell rather than an application opcode.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| 오퍼랜드 | 설명 | 데이터 타입 |
| --- | --- | --- |
| D | On상태를 유지시키고자 하는 접점 / 워드 디바이스의 비트 접점 | BIT |

This source table is not a separately reviewed writer rule. Retain shared variant distinctions and let the current writer reject unsupported operands.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 입력조건이 On 되면 지정출력 접점을 On상태로 유지시켜 입력이 Off되어도 출력이 On상태를 유지합니다. 지정출력 접점이 워드 디바이스의 비트 접점이라면 해당 비트를 1로 셋(SET)합니다.
- 셋(SET) 명령으로 On된 접점은 RST명령으로 Off시킬 수 있습니다.
- 셋(SET) Syy.xx에 대한 설명은 4.6 순차후입 우선 명령을 참조하시기 바랍니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| 셋(SET) | 1 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SET source figure 1](assets/063131cc47d1e8ef-NewItem2561.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `452set.htm` — §4.5.2; source SHA-256 `4d182f3281c479e3688e877cadd1abbba19ebbff4795fb9237b76c7eb9f80b0a`.

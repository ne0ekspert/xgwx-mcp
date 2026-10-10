---
name: "xgk-frst"
description: "Use FRST with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4372frst.htm"]
---

# FRST

Manual section(s): 4.37.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FRST`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `FRST`; the catalog records opcode `1062` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | F영역 중 XGK는F10240 ~ F2047F, XGB는 F2000 ~ F255F 영역 | BIT | F | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- FRST
- D로이 명령은 특수 릴레이 영역인 F영역 중 XGK는F10240 ~ F2047F, XGB는 F2000 ~ 255F 영역 사이의 비트를 리셋(Reset)하는 명령입니다.
- XGK는F10240 ~ F2047F, XGB는 F2000 ~ 255F 영역의 BIT는 셋(Set)시켜도 1스캔 후에는 자동으로 리셋(Reset)되므로 별도로 FRST명령을 사용할 필요가 없습니다.

| 플래그 이름 | 데이터 크기 | 접점 | 접점 | 리셋 동작 |
| --- | --- | --- | --- | --- |
| 플래그 이름 | 데이터 크기 | XGK | XGB | 리셋 동작 |
| _RTC_WR | BIT | F10240 | F2000 | 자동 리셋(Reset) 영역 |
| _SCAN_WR | BIT | F10241 | F2001 | 자동 리셋(Reset) 영역 |
| _CHK_ANC_ERR | BIT | F10242 | F2002 | 자동 리셋(Reset) 영역 |
| _CHK_ANC_WAR | BIT | F10243 | F2003 | 자동 리셋(Reset) 영역 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FRST | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FRST source figure 1](assets/6b88ab1d9c8e9e3d-NewItem3460.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4372frst.htm` — §4.37.2; source SHA-256 `bed07c23b11919529d9cf7d4d47a9f8cacf8627681272b7df761432b69a0a043`.
- Additional behavior/operand evidence: `4372frst.htm`.

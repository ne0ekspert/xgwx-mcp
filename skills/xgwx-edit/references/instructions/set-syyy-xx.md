---
name: "xgk-set-syyy-xx"
description: "Advance a step relay group in sequential order."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["461setsyyyxx.htm"]
---

# SET Syyy.xx

Advance a step relay group in sequential order.

Manual section(s): 4.6.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SET Syyy.xx`.

## Editing route

No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.

This manual form targets a specialized step-sequence S record, not the generic SET/OUT device coil.
No matching specialized public XGK ladder writer route is exposed; do not substitute edit_ladder_cell or an SFC XML operation.

## Behavior

- The target step turns On only while the immediately preceding step in the same group is On and the target input is On; the preceding step then turns Off.
- The active step latches even after input Off. Only one step is active in a group.
- At initial RUN, Syyy.00 is On; enabling Syyy.00 clears the sequential group.

## Instruction constraints

- yyy is the group number 0..127 and xx is the step number 0..99. The operand is a BIT step-relay contact.

## Source corrections

- This is the S-device sequential-control instruction; do not equate its mnemonic template with arbitrary SFC graph-step insertion.

## Operands

| 오퍼랜드 | 설 명 | 데이터 타입 |
| --- | --- | --- |
| Syyy.xx | S디바이스 접점으로, yyy는 조 번호를, xx는 스텝 번호를 나타냅니다. 조 번호는 0~127까지, 스텝 번호는 0~99까지 사용 가능합니다. | BIT |

This source table is not a separately reviewed writer rule. Retain shared variant distinctions and let the current writer reject unsupported operands.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 동일 조내에서 바로 이전의 스텝번호가 On되어 있는 상태에서 현재 스텝번호의 입력조건 접점 상태가 On되면 현재 스텝번호가 On되고, 이전 스텝번호는 Off됩니다.
- 현재 스텝번호가 On되면 자기 유지되어 입력 접점이 Off되어도 On되어진 상태를 유지합니다.
- 입력조건 접점이 동시 On되어도 한 조내에서는 한 스텝번호만이 On되어 집니다.
- 초기 Run시 Syyy.00은 On되어 있습니다.
- 셋(SET) Syyy.xx 명령은 Syyy.00의 입력 접점을 On시킴으로써 클리어됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| 셋(SET) | 1 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SET Syyy.xx source figure 1](assets/4ae27a8c49c2d25a-NewItem2582.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `461setsyyyxx.htm` — §4.6.1; source SHA-256 `be09f7a69870a0d2a5cac782460090b5d7d447cd42134ee7aff66d191e01c5da`.

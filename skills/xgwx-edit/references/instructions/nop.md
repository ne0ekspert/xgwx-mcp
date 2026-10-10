---
name: "xgk-nop"
description: "Use NOP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["481nop.htm"]
---

# NOP

Manual section(s): 4.8.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `NOP`.

## Editing route

No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.

No matching public instruction insertion or basic element route is exposed; preserve existing control-flow/terminal records.

## Operands

No unambiguous operand table is available for this exact form; do not infer types or writable regions.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 무처리(No Operation)명령으로 해당 프로그램의 연산결과에 아무런 영향을 주지 않습니다.
- 니모닉 프로그램에서만 사용되는 명령어입니다.
- NOP 사용 목적은 시퀸스 프로그램의 디버깅용이며 일시적으로 스텝 수를 유지하면서 명령어를 제거하기 위해 사용됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| NOP | 1 | - | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- NOP 명령은 기종에 따라 명령어 처리 시간은 다르지만 처리하는 데는 시간이 소요되므로 삭제를 하면 사용자 프로그램 처리시간(Scan time)을 단축시킬 수 있습니다.
- NOP명령은 래더에서는 입력 할 수 없으며 니모닉에서 등록된 NOP은 래더화면에서는 표시되지 않지만 스텝수는 포함해서 표시합니다.

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `481nop.htm` — §4.8.1; source SHA-256 `39d01c9c92a0b5eb1c251d4cb8c0c0783d18034c4c590b2c909e91681f6bda93`.

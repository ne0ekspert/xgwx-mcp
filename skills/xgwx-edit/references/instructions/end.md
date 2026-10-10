---
name: "xgk-end"
description: "Mark the end of the cyclic program."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["471end.htm"]
---

# END

Mark the end of the cyclic program.

Manual section(s): 4.7.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `END`.

## Editing route

No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.

No matching public instruction insertion or basic element route is exposed; preserve existing control-flow/terminal records.

## Behavior

- After END processing, execution returns to step 0000. Missing END causes program error E4000.

## Instruction constraints

- Ensure the main program has END; subroutine bodies described by CALL/SBRT/RET are placed after it.

## Source corrections

- The END page contains a copied Syyy.xx operand table. END has no operand according to its instruction matrix; do not add a step-relay operand.

## Operands

No unambiguous operand table is available for this exact form; do not infer types or writable regions.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 프로그램 종료를 표시합니다.
- END 명령 처리 후 0000 스텝으로 돌아가 처리합니다.
- END 명령은 반드시 프로그램의 마지막에 입력해야 합니다. 입력하지 않으면 ‘오류 E4000 : END명령어가 존재하지 않습니다.’ 에러가 발생합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| END | 1 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![END source figure 1](assets/212a43ba835e7f4b-NewItem2591.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `471end.htm` — §4.7.1; source SHA-256 `5e6f466e4ec4f7cf3223f1d8c2c61c8aff253c8a321dbea5ee18bb73540d458d`.

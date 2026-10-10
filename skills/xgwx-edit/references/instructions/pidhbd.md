---
name: "xgk-pidhbd"
description: "Combine forward and reverse PID loops for XGB hybrid control."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42810pidhbd.htm"]
---

# PIDHBD

Combine forward and reverse PID loops for XGB hybrid control.

Manual section(s): 4.28.10. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `PIDHBD`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `PIDHBD`; the catalog records opcode `1060` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- F selects the forward loop and R the reverse loop, each in 0..15.

## Instruction constraints

- Do not also control the selected loops with PIDRUN or another PID instruction; duplicate use can cause incorrect control.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | F | 정동작 PID 연산을 수행할 루프 번호 (0~15) | WORD | None | Allowed |
| 2 | R | 역동작 PID 연산을 수행할 루프 번호 (0~15) | WORD | None | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- PIDHBD (PID Hybrid)
- 사용자가 PID형식에 맞추어 저장한 정동작, 역동작 파라미터 또는 K영역(워드로 K1200 ~ K1850)의 PID루프를 연결하여 정/역 혼합운전을 가동합니다.
- K 디바이스 PID 파라미터 영역 각 루프의 K디바이스 내의 파라미터위치 할당은 PIDRUN명령을 참조하시기 바랍니다.
- Quick Start
- PIDHBD 명령을 사용할 때는 반드시 정동작 루프와 역동작 루프번호를 올바르게 지정해야 합니다.
- PIDHBD 명령으로 사용된 루프를 PIDRUN등 다른 명령에서 사용하는 경우, 제어동작이 이상 동작 할 수 있으므로 주의하시기 바랍니다.

## Manual applicability and restrictions

- 적용 기종 : XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| PIDHBD | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![PIDHBD source figure 1](assets/7a393dedc90c7908-NewItem3337.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42810pidhbd.htm` — §4.28.10; source SHA-256 `bdb33c877799e707ff4b1de028d4b85261693fe903f53dfc487bd6406f9c36c6`.
- Additional behavior/operand evidence: `42810pidhbd.htm`.

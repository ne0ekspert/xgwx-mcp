---
name: "xgk-tmon"
description: "Produce a non-retriggered monostable timer interval."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["496tmon.htm"]
---

# TMON

Produce a non-retriggered monostable timer interval.

Manual section(s): 4.9.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TMON`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `TMON`; the catalog records opcode `919` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Input On turns the contact On and starts counting down from the preset.
- After starting, input On/Off changes do not interrupt the countdown. Zero turns the contact Off; RST clears the output and value.

## Instruction constraints

- T selects the timer device; t is its WORD preset. The delay is the preset multiplied by the base time configured for that timer number.
- Available base times are 0.1 ms, 1 ms, 10 ms, and 100 ms; XGB does not support 0.1 ms.
- The same timer number cannot be used twice, even when indexed. Indexed timer/preset operands are limited to Z0..Z3; an indexed RST must match the timer operand form.

## Source corrections

- The initial instruction/device table incorrectly says TMR. The page title, function description, and timer overview identify TMON.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | T | 사용하고자 하는 타이머 접점 | WORD | T | Not allowed |
| 2 | t | 타이머의 설정치를 나타내고 정수나 워드 디바이스 지정 가능 설정시간 = 기본주기 (0.1ms:XGB는 지원안함, 1ms, 10ms, 100ms) x 설정치( t ) | WORD | PMK, U, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 입력조건이 On되는 순간 타이머 출력이 On되고 타이머의 현재값이 설정값으로부터 감소하기 시작하여 0이 되면 타이머 출력은 Off됩니다.
- 타이머 출력이 On된 후 입력조건이 On, Off변화를 하여도 입력조건과 관계 없이 감산은 계속합니다.
- 리셋(Reset) 입력조건이 성립하면 타이머 접점은 Off되고 현재값은 0이 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| TMR | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TMON source figure 1](assets/b2a6faa73374217c-NewItem2613.png)

![TMON source figure 2](assets/64210ea77c23eca4-NewItem2621.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `496tmon.htm` — §4.9.6; source SHA-256 `8a3fec788aa22e18b4a6cfb5a35512c62853439e5d1cfde9554f1896f694e5bd`.
- Additional behavior/operand evidence: `491.htm`, `496tmon.htm`.

---
name: "xgk-tmr"
description: "Accumulate enabled time across separate On intervals."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["495tmr.htm"]
---

# TMR

Accumulate enabled time across separate On intervals.

Manual section(s): 4.9.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TMR`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `TMR`; the catalog records opcode `917` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- The value increases while the input is On and is retained while Off; reaching the preset turns the contact On.
- RST clears the contact and current value. Power-loss retention requires a nonvolatile timer area.

## Instruction constraints

- T selects the timer device; t is its WORD preset. The delay is the preset multiplied by the base time configured for that timer number.
- Available base times are 0.1 ms, 1 ms, 10 ms, and 100 ms; XGB does not support 0.1 ms.
- The same timer number cannot be used twice, even when indexed. Indexed timer/preset operands are limited to Z0..Z3; an indexed RST must match the timer operand form.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | T | 사용하고자 하는 타이머 접점 | WORD | T | Not allowed |
| 2 | t | 타이머의 설정치를 나타내고 정수나 워드 디바이스 지정 가능 설정시간 = 기본주기 (0.1ms:XGB는 지원안함, 1ms, 10ms, 100ms) x 설정치(t) | WORD | PMK, U, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 입력조건이 온(On)되는 동안 현재치가 증가하여 누적된 값이 타이머의 설정시간에 도달하면 타이머 접점이 On됩니다.
- 적산 타이머는 정전시도 타이머 값을 유지하므로 PLC 야간 정전에도 이상없습니다. (불휘발성 영역 사용의 경우)
- 리셋(Reset) 입력조건이 성립되면 타이머 접점은 Off되고 현재치는 0이 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| TMR | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TMR source figure 1](assets/b2a6faa73374217c-NewItem2613.png)

![TMR source figure 2](assets/4c49af7a21237b05-NewItem2614.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `495tmr.htm` — §4.9.5; source SHA-256 `bf1a2cf38fb9f64bad0d2f82f3a13680f92b90f8a3696e30044863c7d6b127d3`.
- Additional behavior/operand evidence: `491.htm`, `495tmr.htm`.

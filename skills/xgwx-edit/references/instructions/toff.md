---
name: "xgk-toff"
description: "Keep a timer contact On for the preset interval after the input turns Off."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["493toff.htm"]
---

# TOFF

Keep a timer contact On for the preset interval after the input turns Off.

Manual section(s): 4.9.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TOFF`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `TOFF`; the catalog records opcode `915` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Input On loads the preset and turns the contact On.
- After input Off, the value counts down; zero turns the contact Off. RST clears the contact and value.

## Instruction constraints

- T selects the timer device; t is its WORD preset. The delay is the preset multiplied by the base time configured for that timer number.
- Available base times are 0.1 ms, 1 ms, 10 ms, and 100 ms; XGB does not support 0.1 ms.
- The same timer number cannot be used twice, even when indexed. Indexed timer/preset operands are limited to Z0..Z3; an indexed RST must match the timer operand form.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | T | 사용하고자 하는 타이머 접점 | WORD | T | Not allowed |
| 2 | t | 타이머의 설정치를 나타내고 정수나 워드 디바이스 지정 가능 설정시간 = 기본주기(0.1ms:XGB는 지원안함, 1ms, 10ms, 100ms) x 설정치(t) | WORD | PMK, U, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- RST(리셋)
- 입력조건이 On되는 순간 성립되는 동안 타이머의 현재치는 설정치가 되며 출력은 On 됩니다.
- 입력조건이 Off되면 타이머 현재치가 설정치로부터 감산되어 현재치가 0이 되는 순간 출력이 Off됩니다.
- 리셋(Reset) 명령을 만나면 타이머 출력은 Off되고 현재치는 0이 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| TOFF | 2~3 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TOFF source figure 1](assets/df03f413a8a645ab-NewItem2605.png)

![TOFF source figure 2](assets/88f4d02affa89e13-NewItem2606.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `493toff.htm` — §4.9.3; source SHA-256 `1297bfd188ac9fe8bac287c3fe7414ed4b7192d433cfd2193564165db53738c6`.
- Additional behavior/operand evidence: `491.htm`, `493toff.htm`.

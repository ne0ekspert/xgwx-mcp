---
name: "xgk-pwm"
description: "Emit a pulse train with a configured period and Off duty on XGB built-in positioning."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44148pwm.htm"]
---

# PWM

Emit a pulse train with a configured period and Off duty on XGB built-in positioning.

Manual section(s): 4.41.48. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `PWM`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `PWM`; the catalog records opcode `1064` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- ax=0/1 selects X/Y; n1 is the period in 1..20,000 ms; n2 is Off duty 0..100 percent. Pulses run while the enable is On without updating current position.

## Instruction constraints

- During PWM, other positioning commands and external upper/lower-limit inputs do not operate; STP and EMG do not stop this output. Control the PWM enable and verify hardware support.
- The body describes XGB built-in positioning and lists CPU hardware/OS minimums; do not assume every CPU in the broad heading supports it.

## Source corrections

- The introductory sentence describes position movement and is copied from another instruction. The function section defines PWM and Off duty.
- The example also refers to DST absolute/relative position settings; this is copied position-operation prose, not a condition for the PWM duty/period instruction.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축 (0:X 축, 1:Y 축) | WORD | PMK, L, Z, D, R | Allowed |
| 3 | n1 | 출력 주기(ms):1~20,000(ms) | WORD | PMK, L, Z, D, R | Allowed |
| 4 | n2 | Off 듀티비:0~100(%) | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 이 명령은 XGB 내장 위치결정에 PWM 출력 지령을 내리는 명령입니다.
- 입력조건이 On인 동안 XGB 위치 결정의 ax로 지정된 축에 n1에 설정된 주기와 n2에 설정된 Off Duty로 펄스열을 출력합니다.
- PWM 출력 중에는 현재 위치 어드레스는 변화하지 않고 펄스출력 중 비트(X축:K04200, Y축:K04300)와 정속중 비트(X축:K0420D, Y축:K0430D)가 On됩니다.
- 명령으로 펄스폭 변조 출력중인 경우는 다른 위치결정 명령이 동작하지 않습니다. 또, 상한, 하한 리미트와 같은 외부 입력 접점도 동작하지 않기 때문에 주의하시기 바랍니다.
- PWM 명령으로 펄스폭 변조 출력중인 경우는 STP, EMG 명령에 의한 정지 동작이 실행되지 않습니다.
- 제품별 PWM 지원가능 버전:
- (1) XBM-DNxxS:H/W V2.0 이상, O/S V3.10 이상
- (2) XBC-DN/DPxxH:O/S V2.03 이상
- (3) XBC-DN/DpxxSU:O/S V1.10 이상
- (4) XBC-DN/DP32U, XBC-DN32H 기종은 버전 상관 없음

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM
- 만약 ax(명령 축)로 지정된 값이 0또는 1이외의 값이 입력된 경우는 에러 플래그(F110)를 셋(set)하고 명령을 실행하지 않습니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| PWM | 4~7 | O | - | - |
| PWM |  |  |  |  |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![PWM source figure 1](assets/0e06294ffa546e9d-NewItem3780.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44148pwm.htm` — §4.41.48; source SHA-256 `09e79bb9adf332ca574774d689de339a5d6f0a858fea2e8e4fd1847856246613`.
- Additional behavior/operand evidence: `44148pwm.htm`.

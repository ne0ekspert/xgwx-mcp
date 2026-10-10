---
name: "xgk-sin"
description: "Use SIN with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4271sinsinp.htm"]
---

# SIN

Manual section(s): 4.27.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SIN`.

Execution form: the source explicitly pairs this command as `(P)`; `SIN` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `SIN`; the catalog records opcode `319` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | Sine 연산의 각도 입력값 ( Radian ) | LREAL | PMK, L, T, C, U, N, D, R | Allowed |
| 2 | D | 연산결과를 저장할 디바이스 번호 | LREAL | PMK, L, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- SIN (Sine)
- S로 지정된 영역의 데이터 값을 SIN연산을 해서 D에 저장합니다. 이때, S와 D의 데이터 타입은 배장형 실수이고, 내부 연산은 배장형 실수로 변환해서 처리합니다.
- 입력값은 라디안값이며, 각도에서 라디안 변환은 RAD를 참고하세요.
- S의 값이 1.047…(π/3 rad = 60° )일 때 연산 결과는 0.8660… ()가 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| SIN(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SIN source figure 1](assets/08e566afbd1d0278-NewItem3262.png)

![SIN source figure 2](assets/f6a80396e5d3f5e8-NewItem3263.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4271sinsinp.htm` — §4.27.1; source SHA-256 `ff17904e0e8d1728999664bc8046f9b4782fd70cecb4c14115cb20f2dcecec58`.
- Additional behavior/operand evidence: `4271sinsinp.htm`, `245.htm`.

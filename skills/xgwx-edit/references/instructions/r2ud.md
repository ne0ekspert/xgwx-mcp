---
name: "xgk-r2ud"
description: "Use R2UD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4139r2ur2upr2udr2udp.htm"]
---

# R2UD

Manual section(s): 4.13.9. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `R2UD`.

Execution form: the source explicitly pairs this command as `(P)`; `R2UD` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `R2UD`; the catalog records opcode `1457` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 실수형 데이터가 저장되어진 영역번호 또는 실수형 데이터 | REAL | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | 무보호 정수형 데이터 형태로 변환된 데이터를 저장할 디바이스 위치 | UINT/UDINT | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- R2U ( Real to Unsigned Integer )
- R2I는 S+1,S로 지정된 단장형 실수(32비트)를 16비트 무보호 정수형 데이터로 변환하여 D에 저장합니다.
- S+1,S로 지정된 단장형 실수의 값이 0~65,535 범위를 벗어날 경우 연산에러가 발생합니다.
- 결과값은 입력값이 65,535보다 클 경우는 65,535이 저장되고, 입력값이 0보다 작을 경우는 0이 저장됩니다.
- 소수점 이하의 값은 버려집니다.
- R2UD ( Real to Double Unsigned Integer )
- R2UD는 S+1,S로 지정된 단장형 실수(32비트) 데이터를 배장형 무보호 정수(32비트)로 변환하여 D+1,D에 저장합니다.
- S+1,S로 지정된 단장형 실수의 값이 0~4,294,967,295 범위를 벗어날 경우 연산 에러가 발생합니다.
- 결과값은 단장형 실수의 값이 4,294,967,295 보다 클 경우는 4,294,967,295 이 저장되고, 단장형 실수의 값이 0보다 작을 경우는 0이 저장됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | R2U명령 사용시, S로 지정된 단장형 실수값이 0~65,535 범위를 벗어날 때. R2UD명령 사용시, S로 지정된 단장형 실수값이 0~4,294,967,295 범위를 벗어날 때. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)
- R2U명령 사용시, S로 지정된 단장형 실수값이 0~65,535 범위를 벗어날 때 연산 에러(F110)가 발생합니다.
- R2UD명령 사용시, S로 지정된 단장형 실수값이 0~4,294,967,295 범위를 벗어날 때 연산 에러(F110)가 발생합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| R2U(P) R2UD(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![R2UD source figure 1](assets/6c0bf7199e828fd8-NewItem2770.png)

![R2UD source figure 2](assets/f5b29b4f4ee76837-NewItem2772.png)

![R2UD source figure 3](assets/6b72674658beedf3-NewItem2773.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4139r2ur2upr2udr2udp.htm` — §4.13.9; source SHA-256 `db8655327d18f88bceaf02c0c1c6a44d273d2a6c8d7ca5fe3c750b060526569f`.
- Additional behavior/operand evidence: `4139r2ur2upr2udr2udp.htm`, `245.htm`.

---
name: "xgk-r2ip"
description: "Use R2IP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4133r2ir2ipr2dr2dp.htm"]
---

# R2IP

Manual section(s): 4.13.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `R2IP`.

Execution form: the source explicitly pairs this command as `(P)`; `R2IP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `R2IP`; the catalog records opcode `444` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 실수형 데이터가 저장되어진 영역번호 또는 실수형 데이터 | REAL | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | 정수형 데이터 형태로 변환된 데이터를 저장할 디바이스 위치 | INT | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- R2I(Real to Integer)
- R2I는 S+1,S로 지정된 단장형 실수(32비트)를 16비트 정수형 데이터로 변환하여 D에 저장합니다.
- S+1,S로 지정된 단장형 실수의 값이 -32,768~32,767 범위를 벗어날 경우 연산에러가 발생합니다.
- 결과값은 입력값이 32,767보다 클 경우는 32,767이 저장되고, 입력값이 -32,768보다 작을 경우는 -32,768이 저장됩니다.
- 소수점 이하의 값은 반올림한 후에 버려집니다.
- R2D(Real to Double Integer)
- R2D는 S+1,S로 지정된 단장형 실수(32비트) 데이터를 배장형 정수(32비트)로 변환하여 D+1,D에 저장합니다.
- S+1,S로 지정된 단장형 실수의 값이 -2,147,483,648~2,147,483,647 범위를 벗어날 경우 연산 에러가 발생합니다.
- 결과값은 단장형 실수의 값이 2,147,483,647보다 클 경우는 2,147,483,647이 저장되고, 단장형 실수의 값이 -2,147,483,648보다 작을 경우는 -2,147,483,648이 저장됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | R2I명령 사용시, S로 지정된 단장형 실수값이 -32,768~32,767 범위를 벗어날 때. R2D명령 사용시, S로 지정된 단장형 실수값이 -2,147,483,648~2,147,483,647 범위를 벗어날 때. | F110 |

## Manual applicability and restrictions

- R2I명령 사용시, S로 지정된 단장형 실수값이 -32,768~32,767 범위를 벗어날 때 연산 에러(F110)가 발생합니다.
- R2D명령 사용시, S로 지정된 단장형 실수값이 -2,147,483,648~2,147,483,647 범위를 벗어날 때 연산 에러(F110)가 발생합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| R2I(P) R2D(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![R2IP source figure 1](assets/3e26abc50d7a062f-NewItem2743.png)

![R2IP source figure 2](assets/f5b29b4f4ee76837-NewItem2745.png)

![R2IP source figure 3](assets/6b72674658beedf3-NewItem2746.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4133r2ir2ipr2dr2dp.htm` — §4.13.3; source SHA-256 `4d5274cf9dce5c47896c48401c14e9d97645412240899ce8949ca71a500862a3`.
- Additional behavior/operand evidence: `4133r2ir2ipr2dr2dp.htm`, `245.htm`.

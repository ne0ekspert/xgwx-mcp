---
name: "xgk-atanp"
description: "Use ATANP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4276atanatanp.htm"]
---

# ATANP

Manual section(s): 4.27.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ATANP`.

Execution form: the source explicitly pairs this command as `(P)`; `ATANP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `ATANP`; the catalog records opcode `746` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | Arc Tangent 연산을 할 TAN값이 저장된 디바이스 번호 | LREAL | PMK, L, T, C, U, N, D, R | Allowed |
| 2 | D | 연산결과(라디안값)를 저장할 디바이스 번호 | LREAL | PMK, L, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ATAN (Arc Tangent)
- S로 지정된 영역의 데이터 값을 ATAN연산을 해서 D에 저장합니다. 이때, S와 D의 데이터 타입은 배장형 실수이고, 내부 연산은 배장형 실수로 변환해서 처리합니다.
- 출력값은 라디안값이며, 라디안에서 각도 변환은 DEG를 참고하세요.
- S의 값이 1.0 일 때 연산 결과는 0.7853... ( π/4 rad = 45°)입니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ATAN(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ATANP source figure 1](assets/7686300efc3a5874-NewItem3278.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4276atanatanp.htm` — §4.27.6; source SHA-256 `b97ee2b2e2a01256bf4ffdb739cd8e679c077d33f94275559b38be42ac54be46`.
- Additional behavior/operand evidence: `4276atanatanp.htm`, `245.htm`.

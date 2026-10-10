---
name: "xgk-daterdp"
description: "Use DATERDP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4291daterddaterdp1.htm"]
---

# DATERDP

Manual section(s): 4.29.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DATERDP`.

Execution form: the source explicitly pairs this command as `(P)`; `DATERDP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DATERDP`; the catalog records opcode `774` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- DATERD (Date Read)
- PLC의 날짜와 시간 데이터를 읽어와서 D에 저장합니다.
- 모든 시간 데이터 값은 BCD형태로 제공됩니다.
- 위 그림에서 ‘백년’은 년도에서 천단위와 백단위를 나타냅니다.
- 예를 들어, 현재 PLC시계가 2004년 10월 15일 14시 59분 40초 금요일이라면, 다음과 같은 결과가 저장됩니다.
- 요일 설명 : 0-일요일, 1-월요일, 2-화요일, 3-수요일, 4-목요일, 5-금요일, 6-토요일

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DATERD(P) | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DATERDP source figure 1](assets/f3ea6cd82e730ad2-NewItem3356.png)

![DATERDP source figure 2](assets/eb37ea795848ba79-NewItem3358.png)

![DATERDP source figure 3](assets/04e9087db8772a2b-NewItem3357.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4291daterddaterdp1.htm` — §4.29.1; source SHA-256 `efa3ef96c0fe9a6098a8881bdb94a6938d340305a90ea5cc435a79076d1501d2`.
- Additional behavior/operand evidence: `4291daterddaterdp1.htm`, `245.htm`.

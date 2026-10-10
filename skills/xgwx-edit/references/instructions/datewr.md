---
name: "xgk-datewr"
description: "Use DATEWR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4292datewrdatewrp.htm"]
---

# DATEWR

Manual section(s): 4.29.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DATEWR`.

Execution form: the source explicitly pairs this command as `(P)`; `DATEWR` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DATEWR`; the catalog records opcode `339` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 시간 데이터가 들어있는 디바이스 번호 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DATEWR (Date Write)
- S, S+1, S+2, S+3으로 지정된 영역의 시간 데이터 값으로 PLC의 시계를 설정합니다.
- 모든 시간 데이터 값은 BCD형태로 제공됩니다.
- 요일 설명 : 0-일요일, 1-월요일, 2-화요일, 3-수요일, 4-목요일, 5-금요일, 6-토요일 요일 데이터는 별도로 입력하지 않아도 결과 날짜에 따른 요일이 자동으로 출력됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S로 지정된 시간 데이터의 크기가 해당되는 크기를 벗어날 경우 | F00040 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DATEWR(P) | 2 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DATEWR source figure 1](assets/c3efbae83e03386b-NewItem3361.png)

![DATEWR source figure 2](assets/bf926ee55ef97683-NewItem3362.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4292datewrdatewrp.htm` — §4.29.3; source SHA-256 `6482f819aa79b3a0911254e1ac889a5e28d5221261feda5b56c21a3e833a60f9`.
- Additional behavior/operand evidence: `4292datewrdatewrp.htm`, `245.htm`.

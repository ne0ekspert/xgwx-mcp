---
name: "xgk-neg"
description: "Use NEG with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4351negnegpdnegdnegp.htm"]
---

# NEG

Manual section(s): 4.35.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `NEG`.

Execution form: the source explicitly pairs this command as `(P)`; `NEG` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `NEG`; the catalog records opcode `348` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 부호변환을 하고자하는 영역 | WORD/DWORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- NEG (Word Negative)
- D로 지정된 영역의 내용을 부호 변환하여 D영역에 저장합니다.
- 모니터링 보기 옵션을 Sign으로 볼 때 모니터링 가능하며, 음수로 변환된 값은 Sign연산에서만 유용합니다.
- DNEG ( Double Word Negative)
- D, D+1로 지정된 영역의 내용을 부호 변환하여 D, D+1영역에 저장합니다.

| 구 분 | 실 행 전 | 실 행 후 |
| --- | --- | --- |
| 영 역 | D0010 | D0010 |
| 데이터 | -30 (hFFE2) | 30 (h001E) |

| 구 분 | 실 행 전 | 실 행 후 |
| --- | --- | --- |
| 영 역 | D0010, D0011 | D0010, D0011 |
| 데이터 | -30 (hFFFFFFE2) | 30 (h0000001E) |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| NEG(P) DNEG(P) | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![NEG source figure 1](assets/08c3570a2a15caf9-NewItem3428.png)

![NEG source figure 2](assets/ec6d2c64b4b0f7f4-NewItem3429.png)

![NEG source figure 3](assets/75d12c95b6a5ea54-NewItem3430.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4351negnegpdnegdnegp.htm` — §4.35.1; source SHA-256 `25fa82c812fdf36c89f1ca2b3e3c749f650271afd13e76034d9efc6e4449624c`.
- Additional behavior/operand evidence: `4351negnegpdnegdnegp.htm`, `245.htm`.

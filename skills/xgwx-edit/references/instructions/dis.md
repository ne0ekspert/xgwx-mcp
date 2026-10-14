---
name: "xgk-dis"
description: "Use DIS with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4245disdisp.htm"]
---

# DIS

Manual section(s): 4.24.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DIS`.

Execution form: the source explicitly pairs this command as `(P)`; `DIS` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DIS`; the catalog records opcode `265` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | DIS연산을 실행할 데이터 주소 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D | 연산 결과를 저장할 주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | D로부터 저장될 4bit 데이터 개수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- DIS (Distribute)
- S로 지정한 디바이스의 데이터를 N개의 니블(4 bit)로 나누어 D로 지정한 디바이스부터 차례로 N개만큼 저장합니다.
- N 의 값이 0 이면 명령은 실행되지 않습니다.
- 디바이스 D, D+1, … 에서 하위 1 니블에 분리된 데이터가 채워지고 남은 상위 비트들에는 0이 저장됩니다.
- N이 4를 초과하면 에러 플래그를 셋(Set)합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N 이 4를 초과할 경우 셋(Set)합니다. D부터 N개의 범위가 해당 디바이스의 허용 범위를 초과할 경우 셋(Set)합니다. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| DIS(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DIS source figure 1](assets/ade217f09addd00c-NewItem3096.png)

![DIS source figure 2](assets/0ae4b423da9e7173-NewItem3097.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4245disdisp.htm` — §4.24.5; source SHA-256 `219ddfaee55a14ba8d6a8e83f7b2c565289693e626038db81cd29df63249b253`.
- Additional behavior/operand evidence: `4245disdisp.htm`, `245.htm`.

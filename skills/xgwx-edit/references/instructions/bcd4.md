---
name: "xgk-bcd4"
description: "Use BCD4 with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4122bcd4bcd4pbcd8bcd8p.htm"]
---

# BCD4

Manual section(s): 4.12.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BCD4`.

Execution form: the source explicitly pairs this command as `(P)`; `BCD4` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `BCD4`; the catalog records opcode `374` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | BCD변환하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호의 비트 위치 | NIBBLE/BYTE | PMK, L, D.x, R.x, U | Allowed |
| 2 | D | 변환된 데이터를 저장할 디바이스 번호의 비트 위치 | NIBBLE/BYTE | PMK, L, D.x, R.x, U | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BCD4
- S로 지정된 디바이스의 4비트의 BIN데이터를(0~9) BCD4로 변환하여 D로 지정된 디바이스 영역에 저장.
- S로 지정된 디바이스의 값이 (0~9) 이외일때, 에러플래그(F110)가 셋(Set)됩니다.
- BCD8
- S로 지정된 디바이스의 8비트의 BIN데이터(0~99)를 BCD8로 변환하여 D로 지정된 디바이스 영역에 저장.
- S로 지정된 디바이스의 값이 (0~99) 이외일때, 에러플래그(F110)가 셋(Set)됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | BCD4의 경우, S로 지정된 디바이스의 값이 0~9 이외일때. BCD8의 경우, S로 지정된 디바이스의 값이 0~99 이외일때. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BCD4(P) BCD8(P) | 3~5 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BCD4 source figure 1](assets/a4ab3ab1f358af13-NewItem2717.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4122bcd4bcd4pbcd8bcd8p.htm` — §4.12.2; source SHA-256 `8878be052ef0f4bfe7a70d1f8ee8e2627f864e4aede780ca6e530f3438d7b3ab`.
- Additional behavior/operand evidence: `4122bcd4bcd4pbcd8bcd8p.htm`, `245.htm`.

---
name: "xgk-ascp"
description: "Use ASCP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42612ascascp1.htm"]
---

# ASCP

Manual section(s): 4.26.12. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ASCP`.

Execution form: the source explicitly pairs this command as `(P)`; `ASCP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `ASCP`; the catalog records opcode `714` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 16진 BIN | WORD | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | D | 변환한 문자열을 저장할 위치 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 3 | N | 변환할 문자수 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- ASC (ASCII)
- S로 지정된 영역의 데이터를 cw의 포맷에 따라 ASCII값으로 변환하여 D로 지정된 위치부터 저장합니다.
- S로 지정된 디바이스 번호 이후에 저장되어 있는 BIN 16비트 데이터를 16진수로 취급하여 아스키로 변환하고, D로 지정된 디바이스 번호 이후 N으로 지정된 문자수분의 범위에 저장합니다.
- N으로 문자수를 설정함으로써 S로 지정된 BIN 데이터의 범위 및 D로 지정된 문자열의 저장 디바이스의 범위가 자동적으로 정해집니다.
- 변환할 BIN 데이터가 저장되어 있는 디바이스 범위와 변환한 아스키 데이터를 저장할 디바이스 범위가 중복하고 있는 경우라도 정상적으로 처리합니다.
- N으로 지정된 문자수가 홀수인 경우, 문자열을 저장할 디바이스 범위의 마지막 디바이스 번호의 상위 8비트에는 자동적으로 h00이 저장됩니다.
- N으로 지정된 문자수가 0일 경우에는 변환 처리를 하지 않습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | cw의 포맷 규정이 틀린경우. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ASC(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ASCP source figure 1](assets/a61a6b4c9210da0b-NewItem3228.png)

![ASCP source figure 2](assets/c33b72c513ad7fc9-NewItem3230.png)

![ASCP source figure 3](assets/fe380dbd8380aa1b-NewItem3229.png)

![ASCP source figure 4](assets/6059c2bf7739e949-NewItem3231.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42612ascascp1.htm` — §4.26.12; source SHA-256 `9bdef8b86c35b88409852950644d4dfe2ba240dfd3bfc9ca32b6a56c8d80e30b`.
- Additional behavior/operand evidence: `42612ascascp1.htm`, `245.htm`.

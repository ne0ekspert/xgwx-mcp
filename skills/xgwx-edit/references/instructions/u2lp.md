---
name: "xgk-u2lp"
description: "Use U2LP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4127wtodwwtodwpdwtowdwtowp2.htm"]
---

# U2LP

Manual section(s): 4.13.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `U2LP`.

Execution form: the source explicitly pairs this command as `(P)`; `U2LP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `U2LP`; the catalog records opcode `1473` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 무보호 정수형 데이터가 저장되어진 영역번호 또는 정수형 데이터 | UINT | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | 실수형 데이터 형태로 변환된 데이터를 저장할 디바이스 위치 | REAL/LREAL | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- U2R (Unsigned Integer to Real)
- S로 지정된 16비트 무보호 정수형 데이터를 단장형 실수(32비트)로 변환하여 D+1, D에 저장합니다.
- U2L (Unsigned Integer to Long Real)
- S로 지정된 16비트 무보호 정수형 데이터를 배장형 실수(64비트)로 변환하여 D+3,D+2,D+1,D에 저장합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| U2R(P) U2L(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![U2LP source figure 1](assets/a7a8d23fa7371b50-NewItem2759.png)

![U2LP source figure 2](assets/4bff2a3aadaa76cf-NewItem2760.png)

![U2LP source figure 3](assets/bb88d9556d595910-NewItem2762.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4127wtodwwtodwpdwtowdwtowp2.htm` — §4.13.7; source SHA-256 `90811d2c9f070b5f10f8ae8033af0d67ba1c0a2c693bb9fca046feb36435b9c3`.
- Additional behavior/operand evidence: `4127wtodwwtodwpdwtowdwtowp2.htm`, `245.htm`.

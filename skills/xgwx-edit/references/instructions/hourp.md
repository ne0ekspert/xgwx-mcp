---
name: "xgk-hourp"
description: "Use HOURP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4296hourhourp.htm"]
---

# HOURP

Manual section(s): 4.29.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `HOURP`.

Execution form: the source explicitly pairs this command as `(P)`; `HOURP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `HOURP`; the catalog records opcode `784` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 상하위 바이트 교환을 하게 되는 데이터 또는 데이터가 들어있는 디바이스 번호 | WORD | PMK, L, Z, U, N, D, R | Allowed |
| 2 | D | GSWAP2연산의 결과를 저장할 주소 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- HOUR
- S+1, S영역에 있는 초 데이터를 시간 데이터로 변경하여 D+1, D에 저장합니다.
- 시간 데이터는 BCD값으로 저장됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set). 해당 명령어 결과는 처리되지 않음. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| HOUR(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![HOURP source figure 1](assets/74de8e9d5e4d2f65-NewItem3379.png)

![HOURP source figure 2](assets/3c14c70ee507eea6-NewItem3381.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4296hourhourp.htm` — §4.29.7; source SHA-256 `05251a0f137d86473b8c69d7bbedbe6f7eff88884b1f69360d7856ccdebd9c80`.
- Additional behavior/operand evidence: `4296hourhourp.htm`, `245.htm`.

---
name: "xgk-wtobp"
description: "Use WTOBP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4247wtobwtobp.htm"]
---

# WTOBP

Manual section(s): 4.24.7. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `WTOBP`.

Execution form: the source explicitly pairs this command as `(P)`; `WTOBP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `WTOBP`; the catalog records opcode `452` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | WORD데이터가 저장되어진 영역번호 또는 WORD데이터 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | BYTE형으로 변환된 데이터를 저장할 영역의 시작 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | BYTE형으로 변환한 바이트의 개수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- WTOB
- S부터 각 워드 데이터를 2개의 바이트로 나눠서 나누어진 바이트N개를 D부터 저장합니다. 이때 상위바이트는 0으로 채워지고 하위바이트에 나눠진 바이트 값이 저장됩니다.
- N=0일 경우 명령을 수행하지 않습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S나 D가 해당 디바이스의 허용 범위를 초과할 경우 셋(Set)합니다. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| WTOB(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![WTOBP source figure 1](assets/f6e4c86f28bdbd3b-NewItem3103.png)

![WTOBP source figure 2](assets/3d1aa3d12169be18-NewItem3104.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4247wtobwtobp.htm` — §4.24.7; source SHA-256 `bf6a565bd3aee1c9834a006cc41604d099cbfd05ebc3cc5e054133ae1074a363`.
- Additional behavior/operand evidence: `4247wtobwtobp.htm`, `245.htm`.

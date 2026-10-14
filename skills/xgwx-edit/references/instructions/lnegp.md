---
name: "xgk-lnegp"
description: "Use LNEGP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4352rnegrnegplneglnegp.htm"]
---

# LNEGP

Manual section(s): 4.35.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `LNEGP`.

Execution form: the source explicitly pairs this command as `(P)`; `LNEGP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `LNEGP`; the catalog records opcode `800` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 부호 변환을 하고자 하는 영역 | REAL/LREAL | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- RNEG (Real Negative)
- D로 지정된 영역의 내용을 부호 변환하여 D영역에 저장합니다.
- RNEG는 단장형 실수의 부호 반전에 사용됩니다.
- LNEG (Long Real Negative)
- LNEG 는 배장형 실수의 부호반전에 사용됩니다.

| 실행 전 | 실행 후 |
| --- | --- |
| -3.383240094 | 3. 383240094 |

| 실행 전 | 실행 후 |
| --- | --- |
| -3.3832400941234567 | 3. 3832400941234567 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| RNEG(P) LNEG(P) | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![LNEGP source figure 1](assets/926c483d2a90a10f-NewItem3432.png)

![LNEGP source figure 2](assets/51e83a22ecb8495d-NewItem3433.png)

![LNEGP source figure 3](assets/a73a93fe86021d2e-NewItem3434.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4352rnegrnegplneglnegp.htm` — §4.35.2; source SHA-256 `8c7e668713a578983415cdf71c6fdf4baf35b19b83f7185f5966e3f533fd17de`.
- Additional behavior/operand evidence: `4352rnegrnegplneglnegp.htm`, `245.htm`.

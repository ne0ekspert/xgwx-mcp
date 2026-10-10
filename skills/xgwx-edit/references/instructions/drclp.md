---
name: "xgk-drclp"
description: "Use DRCLP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4175rclrclpdrcldrclp.htm"]
---

# DRCLP

Manual section(s): 4.17.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DRCLP`.

Execution form: the source explicitly pairs this command as `(P)`; `DRCLP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DRCLP`; the catalog records opcode `492` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 연산을 수행하게 될 데이터의 주소 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | n | 좌측으로 회전시킬 횟수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- RCL (Rotate Left with Carry)
- 워드 데이터D의 각 비트를 n번씩 좌측으로 비트 회전시키며 최상위 비트 데이터는 캐리 플래그(F112)로 이동하고 원래의 캐리 플래그(F112)는 최하위 비트로 이동합니다. (1워드 내에서 회전)
- DRCL (Double Rotate Left with Carry)
- D 와 D+1 의 32 bit 데이터를 좌측으로 캐리 플래그를 포함하지 않고 n bit 회전합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 캐리 | 회전 중 캐리가 발생하면 캐리 플래그를 셋(Set)합니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| RCL(P) DRCL(P) | 2~4 | - | - | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DRCLP source figure 1](assets/af55ebfeb6a32a99-NewItem2890.png)

![DRCLP source figure 2](assets/b07c897eba3d5384-NewItem2891.png)

![DRCLP source figure 3](assets/8002568e637a9f0d-NewItem2892.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4175rclrclpdrcldrclp.htm` — §4.17.5; source SHA-256 `1716eae6e3a6e58e25c45f9be9dc8e8e06a67432e0753984e85851e658b28321`.
- Additional behavior/operand evidence: `4175rclrclpdrcldrclp.htm`, `245.htm`.

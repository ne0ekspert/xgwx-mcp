---
name: "xgk-gxchg"
description: "Use GXCHG with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4192gxchggxchgp.htm"]
---

# GXCHG

Manual section(s): 4.19.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GXCHG`.

Execution form: the source explicitly pairs this command as `(P)`; `GXCHG` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `GXCHG`; the catalog records opcode `209` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D1 | D2와 워드단위로 데이터를 교환하게 되는 영역의 시작주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D2 | D1과 워드단위로 데이터를 교환하게 되는 영역의 시작주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | 워드단위로 데이터를 교환하는 개수 | WORD | PMK, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- GXCHG
- D1부터 시작되는 N개의 워드 데이터와 D2로 시작되는 N개의 워드 데이터를 서로 교환합니다.
- D1 과 D2 의 값을 기준으로 증가하면서 N개의 워드만큼 데이터를 교환합니다. 만약 D1 과 D2 가 겹치게 되면 의도하지 않은 결과를 초래할 수 있습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set) | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GXCHG(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GXCHG source figure 1](assets/c2cf1a0e6333721d-NewItem2934.png)

![GXCHG source figure 2](assets/98767f4577fd1fcc-NewItem2936.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4192gxchggxchgp.htm` — §4.19.2; source SHA-256 `f6771f11e0291b03c49362a0b33f8d6661eac62c97bb9cd838958ff663232ac1`.
- Additional behavior/operand evidence: `4192gxchggxchgp.htm`, `245.htm`.

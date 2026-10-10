---
name: "xgk-gswapp"
description: "Use GSWAPP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4194gswapgswapp.htm"]
---

# GSWAPP

Manual section(s): 4.19.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GSWAPP`.

Execution form: the source explicitly pairs this command as `(P)`; `GSWAPP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `GSWAPP`; the catalog records opcode `522` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 상하위 바이트 교환을 하게 되는 데이터의 첫 디바이스 번호 | WORD | PMK, L, U, N, D, R | Not allowed |
| 2 | N | 상하위 바이트 교환을 하게 되는 워드 데이터의 개수 | WORD | PMK, L, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- GSWAP (Group SWAP)
- N개의 워드 안에서 상하위 바이트를 교환합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set) | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GSWAP(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GSWAPP source figure 1](assets/d7881a180f328c62-NewItem2943.png)

![GSWAPP source figure 2](assets/1b2e154558447a58-NewItem2944.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4194gswapgswapp.htm` — §4.19.4; source SHA-256 `947ff2a9c803d066adbd76e15f27afeeec03901475fb0103c4d3715cd6f70f9b`.
- Additional behavior/operand evidence: `4194gswapgswapp.htm`, `245.htm`.

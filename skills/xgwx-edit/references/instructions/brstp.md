---
name: "xgk-brstp"
description: "Use BRSTP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4242brstbrstp.htm"]
---

# BRSTP

Manual section(s): 4.24.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BRSTP`.

Execution form: the source explicitly pairs this command as `(P)`; `BRSTP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BRSTP`; the catalog records opcode `813` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 리셋 시작 위치를 나타내는 디바이스의 번호 | BIT | PMK, L, D.x, U | Not allowed |
| 2 | N | 리셋시킬 비트 개수 | WORD | PMK, L, Z, U, D | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BRST (Bit Reset)
- D로 지정된 비트부터 N개의 비트를 Off로 합니다.
- N의 값이 지정된 비트 접점 영역을 초과할 경우, 에러 플래그가 On됩니다.
- SR명령과 같이 사용하면, SR에서 사용하는 영역을 간단하게 리셋 시킬 수 있습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 값이 D로 지정된 디바이스의 최대 영역을 넘어가도록 지정했을 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BRST(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BRSTP source figure 1](assets/a36942e9a561f31f-NewItem3085.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4242brstbrstp.htm` — §4.24.2; source SHA-256 `7f41a2fdfef9c25d26cde1b6dc2d745fb4efc4532d2bb8b61deadcf811259bb1`.
- Additional behavior/operand evidence: `4242brstbrstp.htm`, `245.htm`.

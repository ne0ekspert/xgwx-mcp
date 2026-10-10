---
name: "xgk-dmux"
description: "Use DMUX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42415muxmuxpdmuxdmuxp.htm"]
---

# DMUX

Manual section(s): 4.24.15. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DMUX`.

Execution form: the source explicitly pairs this command as `(P)`; `DMUX` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DMUX`; the catalog records opcode `279` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 선택할 위치(0 ~ N-1) | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | S2 | 선택할 데이터의 선두 위치 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | D | 선택된 값이 저장될 영역 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 4 | N | 선택할 데이터 범위 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- MUX
- S2부터 N개의 WORD 데이터 중에서 S1에 해당하는 데이터를 D에 전송합니다.
- DMUX
- S2부터 N개의 DWORD 데이터 중에서 S1에 해당하는 데이터를 D에 전송합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N값이 해당 디바이스의 범위를 벗어날 때 데이터 선택 위치가 검색 범위를 벗어날 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| MUX(P) DMUX(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DMUX source figure 1](assets/4ebe8a24e6716445-NewItem3131.png)

![DMUX source figure 2](assets/ac605eca31aaab1d-NewItem3132.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42415muxmuxpdmuxdmuxp.htm` — §4.24.15; source SHA-256 `3d08e49a8811d87f2c6fb146e4adaacd41d36cc8becd80bc8847bad90479350e`.
- Additional behavior/operand evidence: `42415muxmuxpdmuxdmuxp.htm`, `245.htm`.

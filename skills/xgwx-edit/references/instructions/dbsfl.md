---
name: "xgk-dbsfl"
description: "Use DBSFL with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4182bsflbsflpdbsfldbsflp.htm"]
---

# DBSFL

Manual section(s): 4.18.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DBSFL`.

Execution form: the source explicitly pairs this command as `(P)`; `DBSFL` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DBSFL`; the catalog records opcode `201` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 비트 이동을 하고자 하는 디바이스 번호 | WORD/DWORD | PMK, L, T, C, U, N, D, R | Not allowed |
| 2 | n | 워드 데이터 D를 왼쪽으로 bit shift할 횟수 | WORD | PMK, F, L, T, C, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BSFL (Bit Shift Left)
- D의 워드 데이터의 각 비트들을 왼쪽으로 n번 bit shift합니다.
- DBSFL (Double Bit Shift Left)
- D+1,D의 더블 워드 데이터의 각 비트들을 왼쪽으로 n번 bit shift합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 캐리 | 마지막으로 버려진 비트에 따라 캐리 플래그를 On/Off 합니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BSFL(P) DBSFL(P) | 2~4 | - | - | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DBSFL source figure 1](assets/85236fea90accfdd-NewItem2905.png)

![DBSFL source figure 2](assets/b228dab9f3974f09-NewItem2906.png)

![DBSFL source figure 3](assets/3ed9e5c9e3ebe4f3-NewItem2907.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4182bsflbsflpdbsfldbsflp.htm` — §4.18.2; source SHA-256 `15f68f055af5d5ca3e17fb32788b12cf87a2508ea65ddc93d3c51ea0928dceec`.
- Additional behavior/operand evidence: `4182bsflbsflpdbsfldbsflp.htm`, `245.htm`.

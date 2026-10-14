---
name: "xgk-filrd"
description: "Use FILRD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4253filrdfilrdp1.htm"]
---

# FILRD

Manual section(s): 4.25.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FILRD`.

Execution form: the source explicitly pairs this command as `(P)`; `FILRD` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `FILRD`; the catalog records opcode `284` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 데이터 테이블 시작 위치 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 데이터 테이블로부터 읽어온 값을 저장할 위치 | WORD | PMK, L, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- FILRD (Last File Read)
- S로 지정된 데이터 테이블에서 마지막 데이터를 D로 가져옵니다. 테이블의 데이터 개수는 1이 감소되고, 나머지 데이터들은 변하지 않습니다.
- S로 지정된 값은 데이터 테이블 내의 유효 데이터 개수입니다.
- 테이블의 데이터 개수가 1이 감소되면 데이터 테이블 크기+1 에 있는 값은 0으로 채워집니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 데이터 테이블의 범위가 해당 디바이스 범위를 벗어날 때 | F110 |
| 제로 | 데이터 테이블에 데이터가 없을 때. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FILRD(P) | 2~4 | O | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FILRD source figure 1](assets/08f7ecdd958bf75a-NewItem3163.png)

![FILRD source figure 2](assets/9d149f81931628da-NewItem3164.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4253filrdfilrdp1.htm` — §4.25.3; source SHA-256 `6df0a293bb3ec104c4fde6f74bcf0133cfab40d2d81a6b8a429ab6030d959d6e`.
- Additional behavior/operand evidence: `4253filrdfilrdp1.htm`, `245.htm`.

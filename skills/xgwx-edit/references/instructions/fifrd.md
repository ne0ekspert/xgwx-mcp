---
name: "xgk-fifrd"
description: "Use FIFRD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4252fifrdfifrdp1.htm"]
---

# FIFRD

Manual section(s): 4.25.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FIFRD`.

Execution form: the source explicitly pairs this command as `(P)`; `FIFRD` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `FIFRD`; the catalog records opcode `283` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 데이터 테이블 시작위치 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D | 데이터 테이블로부터 읽어온 값을 저장할 위치 | WORD | PMK, L, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- FIFRD( First File Read )
- S로 지정된 데이터 테이블에서 첫번째 데이터를 D로 가져옵니다. 테이블의 데이터 개수는 1이 감소되고, 나머지 데이터들은 1감소된 디바이스 위치로 모두 옮겨집니다.
- S로 지정된 값은 데이터 테이블 내의 유효 데이터 개수입니다.
- 테이블의 데이터 개수가 1이 감소되면 데이터 테이블 크기+1 에 있는 값은 0으로 채워집니다.
- 데이터 개수가 0이면 D에는 0이 저장되고 제로 플래그(F111)이 셋(Set)됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 데이터 테이블의 범위가 해당 디바이스 범위를 벗어날 때 | F110 |
| 제로 | 데이터 테이블에 데이터가 없을 때 | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FIFRD(P) | 2~4 | O | 0 | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FIFRD source figure 1](assets/c888359bfae7f654-NewItem3160.png)

![FIFRD source figure 2](assets/26934b40866f8b55-NewItem3161.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4252fifrdfifrdp1.htm` — §4.25.2; source SHA-256 `61a30c624bf94acaa0fff67bced4b6ebe94edefa97e44bb3f5b2080235c67c95`.
- Additional behavior/operand evidence: `4252fifrdfifrdp1.htm`, `245.htm`.

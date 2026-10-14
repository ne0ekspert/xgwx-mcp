---
name: "xgk-fiinsp"
description: "Use FIINSP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4254fiinsfiinsp1.htm"]
---

# FIINSP

Manual section(s): 4.25.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FIINSP`.

Execution form: the source explicitly pairs this command as `(P)`; `FIINSP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `FIINSP`; the catalog records opcode `668` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 입력할 데이터 값 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 데이터 테이블 시작위치 | WORD | PMK, L, N, D, R | Not allowed |
| 3 | N | 입력된 값을 저장할 위치 | WORD | PMK, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- FIINS (File Insert)
- S로 지정된 값을 D로 지정된 데이터 테이블의 N번째 위치에 삽입합니다. 원래 있던 N번째부터의 데이터는 다음 디바이스 번호로 밀려납니다.
- D로 지정된 값은 데이터 테이블 내의 유효 데이터 갯수입니다.
- N이 0이면 명령어는 동작하지 않습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 데이터 테이블의 범위가 해당 디바이스 범위를 벗어날 때 N 값이 현재 데이터 크기(D로 지정된 값) + 1 보다 클 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FIINS(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FIINSP source figure 1](assets/4961f1bf086a9725-NewItem3166.png)

![FIINSP source figure 2](assets/d92b168c9830509a-NewItem3167.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4254fiinsfiinsp1.htm` — §4.25.4; source SHA-256 `803c399da6505111961bca6d235fdbff4601736815135bc8ba42a882739e12ce`.
- Additional behavior/operand evidence: `4254fiinsfiinsp1.htm`, `245.htm`.

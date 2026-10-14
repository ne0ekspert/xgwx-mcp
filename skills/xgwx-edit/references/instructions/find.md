---
name: "xgk-find"
description: "Use FIND with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42617findfindp1.htm"]
---

# FIND

Manual section(s): 4.26.17. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FIND`.

Execution form: the source explicitly pairs this command as `(P)`; `FIND` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `FIND`; the catalog records opcode `314` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 검색 대상 문자열의 시작 주소 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | S2 | 검색할 문자열의 시작 주소 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 3 | D | 결과를 저장할 주소 | WORD | PMK, L, Z, U, N, D, R | Not allowed |
| 4 | N | 문자열 검색을 시작할 위치 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- FIND
- S1으로 지정된 문자열의 N번째의 문자부터 S2로 시작되는 문자열을 검색하여 첫번째로 일치하는 문자열의 시작 위치를 D에 저장한다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 1.S1, S2의 문자열의 길이가 최대 문자열 크기를 초과하였을 때. 2.N에 지정된 검색 시작 위치가 S1 에 지정된 검색 대상 문자열의 길이보다 클 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FIND(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FIND source figure 1](assets/5ab8dde845aa38c2-NewItem3248.png)

![FIND source figure 2](assets/bea1979740cdc5a5-NewItem3250.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42617findfindp1.htm` — §4.26.17; source SHA-256 `dc35134c76f9aded1ab7938252a23fa135d5e82e06e4ae19ca8c5fa5295b258c`.
- Additional behavior/operand evidence: `42617findfindp1.htm`, `245.htm`.

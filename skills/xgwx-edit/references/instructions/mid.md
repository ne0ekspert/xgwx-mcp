---
name: "xgk-mid"
description: "Use MID with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42615midmidp1.htm"]
---

# MID

Manual section(s): 4.26.15. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `MID`.

Execution form: the source explicitly pairs this command as `(P)`; `MID` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `MID`; the catalog records opcode `312` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 문자열 시작 주소 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | D | 문자열인 연산결과를 저장할 주소 | STRING | PMK, L, Z, U, N, D, R | Not allowed |
| 3 | S2 | S2+0 에 선두 문자의 위치, S2+1 에 가져올 문자열 수 | WORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- MID (Middle)
- S1으로 지정된 디바이스 번호 이후에 저장되어 있는 문자열 데이터의 왼쪽부터 S2로 지정된 위치부터 S2+1로 지정된 문자분의 데이터를 D로 지정된 디바이스 번호 이후에 저장합니다.
- S2+1로 지정된 문자열의 길이가 0이면 D 에는 NULL STRING( “” )이 저장됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 1.변환된 값이 D로 지정된 영역을 초과할 경우 2.S1 의 문자열의 길이가 최대 문자열의 크기를 넘을 때 3.S2+1에 지정된 문자수가 S2+0에 지정된 문자위치와 더했을 때 S1의 문자열의 길이를 넘을 때 4.S2+1 에 지정된 문자 수가 최대 문자열의 크기를 넘을 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| MID(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![MID source figure 1](assets/d66b5cd075bd6476-NewItem3242.png)

![MID source figure 2](assets/24652124c49d10b4-NewItem3243.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42615midmidp1.htm` — §4.26.15; source SHA-256 `d6ded16456e5a9093f35700da9844f614602c0eb00093662eb6fa7b9af30ea23`.
- Additional behavior/operand evidence: `42615midmidp1.htm`, `245.htm`.

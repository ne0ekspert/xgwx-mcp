---
name: "xgk-dmaxp"
description: "Use DMAXP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["42411maxmaxpdmaxdmaxp.htm"]
---

# DMAXP

Manual section(s): 4.24.11. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DMAXP`.

Execution form: the source explicitly pairs this command as `(P)`; `DMAXP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DMAXP`; the catalog records opcode `640` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | MAX연산을 시작할 데이터의 주소 | INT/DINT | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | D | 연산 결과를 저장할 주소 | INT/DINT | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | S로부터 MAX연산을 실행할 워드 개수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- MAX (Maximum)
- 워드 데이터 S로부터 N개 까지의 범위 내에서 최대값을 찾아 D에 저장합니다.
- 대소 비교는 Signed 연산으로 합니다.
- 연산 결과가 제로이면 제로 플래그를 셋(Set)합니다.
- N이 0인 경우 명령어는 실행하지 않습니다.
- DMAX (Double Maximum)
- 더블 워드 데이터 (S+1,S)로부터 N개까지의 범위 내에서 최대값을 찾아 D에 저장합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N값이 해당 디바이스의 범위를 벗어날 때 | F110 |
| 제로 | 연산결과가 제로이면 셋(Set)합니다. | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| MAX(P) DMAX(P) | 4~6 | O | O | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DMAXP source figure 1](assets/44aaf3a4b5b8ffc5-NewItem3115.png)

![DMAXP source figure 2](assets/1612b677109fa754-NewItem3116.png)

![DMAXP source figure 3](assets/37cd17c989c5ba16-NewItem3117.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `42411maxmaxpdmaxdmaxp.htm` — §4.24.11; source SHA-256 `0f5a623aba1d96e5b104d079429b3ac2a4bde6a6552047480da5a79811f8414e`.
- Additional behavior/operand evidence: `42411maxmaxpdmaxdmaxp.htm`, `245.htm`.

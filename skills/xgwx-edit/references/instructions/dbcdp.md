---
name: "xgk-dbcdp"
description: "Use DBCDP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4121bcdbcdpdbcddbcdp.htm"]
---

# DBCDP

Manual section(s): 4.12.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DBCDP`.

Execution form: the source explicitly pairs this command as `(P)`; `DBCDP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DBCDP`; the catalog records opcode `426` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 데이터가 저장되어 있는 디바이스 번호 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | DestinatiOn영역의 디바이스 번호 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BCD(Binary Coded Decimal)
- S로 지정된 디바이스의 BIN데이터(0~h270F)를 BCD로 변환하여 D에 저장합니다.
- DBCD(Double Binary Coded Decimal)
- S+1,S로 지정된 디바이스의 BIN데이터(0~h05F5E0FF)를 BCD로 변환하여, D+1,D에 각각 저장합니다.
- BIN 데이터가 BCD변환하여 표시할 수 있는 범위를 초과하면 에러 플래그(F110)를 셋(Set)합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | BCD(P)의 경우, S의 값이 0~9999(h270F) 이외일 때. DBCD(P)의 경우, S+1,S의 값이 0~99999999(h5F5E0FF) 이외일 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BCD(P) DBCD(P) | 2~4 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DBCDP source figure 1](assets/00f0be2b5402c919-NewItem2710.png)

![DBCDP source figure 2](assets/2ca79d65eb60d24c-NewItem2711.png)

![DBCDP source figure 3](assets/b2d7f88f5a28a412-NewItem2712.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4121bcdbcdpdbcddbcdp.htm` — §4.12.1; source SHA-256 `c6e43be83439d56cc5e7c62b04c04e1d8227dbe0eedf7a9bf40d4819d48260bf`.
- Additional behavior/operand evidence: `4121bcdbcdpdbcddbcdp.htm`, `245.htm`.

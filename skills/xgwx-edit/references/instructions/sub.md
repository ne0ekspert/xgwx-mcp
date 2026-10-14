---
name: "xgk-sub"
description: "Use SUB with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4202subsubpdsubdsubp.htm"]
---

# SUB

Manual section(s): 4.20.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `SUB`.

Execution form: the source explicitly pairs this command as `(P)`; `SUB` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `SUB`; the catalog records opcode `214` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 뺄셈연산을 실행할 데이터 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 뺄셈연산을 실행할 데이터 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | INT | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- SUB (Signed Binary Subtract)
- 워드데이터 S1에서 S2를 감산후 결과를 D(16bit)에 저장합니다.
- 이 때 Signed 연산을 실행합니다.
- 연산 결과가 32,767(h7FFF)을 초과하거나 -32,768(h8000) 미만일 때 캐리 플래그는 셋(Set)되지 않습니다.
- DSUB (Signed Binary Double Subtract)
- 워드데이터 (S1+1, S1)에서 (S2+1,S2)를 감산후 결과를 (D+1,D)에 저장합니다.
- 연산 결과가 2,147,483,647 (h7FFFFFFF)를 초과하거나 -2,147,483,648(h80000000) 미만일 때 캐리 플래그는 셋(Set)되지 않습니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| SUB(P) DSUB(P) | 4~6 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![SUB source figure 1](assets/ac9cdc9bd84b81e5-NewItem2959.png)

![SUB source figure 2](assets/9054dc410b158934-NewItem2960.png)

![SUB source figure 3](assets/b8c1f8fc943145e5-NewItem2961.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4202subsubpdsubdsubp.htm` — §4.20.2; source SHA-256 `bb59d1b3843de3d4e6b7b1374f6a3c636f5e9423d71454ae825a7820b3136d00`.
- Additional behavior/operand evidence: `4202subsubpdsubdsubp.htm`, `245.htm`.

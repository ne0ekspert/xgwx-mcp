---
name: "xgk-gwor"
description: "Use GWOR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4226gworgworp1.htm"]
---

# GWOR

Manual section(s): 4.22.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GWOR`.

Execution form: the source explicitly pairs this command as `(P)`; `GWOR` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `GWOR`; the catalog records opcode `257` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 GWOR연산을 시작하게 되는 데이터의 주소 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | S2 | S1과 GWOR연산을 시작하게 되는 데이터의 주소 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 4 | N | WOR연산을 하는 개수 | WORD | PMK, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- GWOR (Group Word OR)
- S1과 S2에서 부터 워드단위로 논리합(WOR)을 N번 실행하여 그 결과들을 D에 차례로 저장합니다.
- S2에는 h0000 ~ hFFFF(BIN 16비트)의 정수를 지정할 수 있습니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 값이 지정 영역을 벗어날 때 셋(Set)됩니다. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GWOR(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GWOR source figure 1](assets/aa06ed3be450a785-NewItem3036.png)

![GWOR source figure 2](assets/37b182e04cad8f9a-NewItem3037.png)

![GWOR source figure 3](assets/0cbf5a686bba1e43-NewItem3038.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4226gworgworp1.htm` — §4.22.6; source SHA-256 `fd7272201807564129c65eb68036551add6dfed091d4e06f2ab2dd4e5a1a3de3`.
- Additional behavior/operand evidence: `4226gworgworp1.htm`, `245.htm`.

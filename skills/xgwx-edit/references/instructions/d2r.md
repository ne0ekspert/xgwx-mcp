---
name: "xgk-d2r"
description: "Use D2R with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4132d2rd2rpd2ld2lp.htm"]
---

# D2R

Manual section(s): 4.13.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `D2R`.

Execution form: the source explicitly pairs this command as `(P)`; `D2R` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `D2R`; the catalog records opcode `167` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 배장 정수형 데이터가 저장되어진 영역번호 또는 배장 정수형 데이터 | DINT | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | 실수형 데이터 형태로 변환된 데이터를 저장할 디바이스 위치 | REAL | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- D2R(Double Integer to Real)
- D2R(P)은 S+1,S로 지정된 32비트 배장 정수형 데이터를 단장형 실수(32비트)로 변환하여 D+1, D에 저장합니다.
- 32 bit 정수 데이터 값이 단장형 부동 소수점형 실수 데이터의 유효 자리 범위(24 bit)를 초과할 경우, 정확도가 소실되며 부정확 에러 플래그(F0057A)가 셋(Set)됩니다.
- 부정확 에러 플래그가 셋(Set)되어도 PLC 운전상태는 변화가 없습니다.
- D2L(Double Integer to Long Real)
- D2L(P)은 S+1,S로 지정된 32비트 배장 정수형 데이터를 배장형 실수(64비트)로 변환하여 D+3,D+2,D+1,D에 저장합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| D2R(P) D2L(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![D2R source figure 1](assets/2d37db5b1e10e41d-NewItem2739.png)

![D2R source figure 2](assets/1f38602435c2ac4e-NewItem2740.png)

![D2R source figure 3](assets/8c1488576e36c0cd-NewItem2741.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4132d2rd2rpd2ld2lp.htm` — §4.13.2; source SHA-256 `62809c63011ca5e3a34ee8e1538312e89ded4c43d7d53a061b3b2c673c13868c`.
- Additional behavior/operand evidence: `4132d2rd2rpd2ld2lp.htm`, `245.htm`.

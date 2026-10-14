---
name: "xgk-ud2rp"
description: "Use UD2RP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4138ud2rud2rpud2lud2lp.htm"]
---

# UD2RP

Manual section(s): 4.13.8. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `UD2RP`.

Execution form: the source explicitly pairs this command as `(P)`; `UD2RP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `UD2RP`; the catalog records opcode `1472` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 배장 무보호 정수형 데이터가 저장되어진 영역번호 또는 배장 정수형 데이터 | UDINT | PMK, F, L, T, C, U, N, D, R | Allowed |
| 2 | D | 실수형 데이터 형태로 변환된 데이터를 저장할 디바이스 위치 | REAL/LREAL | PMK, L, T, C, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- UD2R (Double Unsigned Integer to Real)
- D2R(P)은 S+1,S로 지정된 32비트 배장 무보호 정수형 데이터를 단장형 실수(32비트)로 변환하여 D+1, D에 저장합니다.
- 32 bit 무보호 정수 데이터 값이 단장형 부동 소수점형 실수 데이터의 유효 자리 범위(24 bit)를 초과할 경우, 정확도가 소실되며 부정확 에러 플래그(F0057A)가 셋(Set)됩니다.
- 부정확 에러 플래그가 셋(Set)되어도 PLC 운전상태는 변화가 없습니다.
- UD2L (Double Unsigned Integer to Long Real)
- UD2L(P)은 S+1,S로 지정된 32비트 배장 무보호 정수형 데이터를 배장형 실수(64비트)로 변환하여 D+3,D+2,D+1,D에 저장합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| UD2R(P) UD2L(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![UD2RP source figure 1](assets/0b01913624298120-NewItem2767.png)

![UD2RP source figure 2](assets/0b64f41fad608131-NewItem2764.png)

![UD2RP source figure 3](assets/55bf3b8b43cad5a6-NewItem2765.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4138ud2rud2rpud2lud2lp.htm` — §4.13.8; source SHA-256 `ec1553b380a373e8a99ef535b07bc27030730654fe6192418a82996809e5ef11`.
- Additional behavior/operand evidence: `4138ud2rud2rpud2lud2lp.htm`, `245.htm`.

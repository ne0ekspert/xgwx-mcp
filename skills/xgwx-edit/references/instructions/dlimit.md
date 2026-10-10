---
name: "xgk-dlimit"
description: "Use DLIMIT with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4281limitlimitpdlimitdlimitp.htm"]
---

# DLIMIT

Manual section(s): 4.28.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DLIMIT`.

Execution form: the source explicitly pairs this command as `(P)`; `DLIMIT` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DLIMIT`; the catalog records opcode `333` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 제어하고자 하는 입력값이 있는 디바이스 번호 | INT/DINT | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | S2 | 출력값의 상한값 | INT/DINT | PMK, L, T, C, Z, U, N, D, R | Allowed |
| 3 | S3 | 출력값의 하한값 | INT/DINT | PMK, L, T, C, Z, U, N, D, R | Allowed |
| 4 | D | 출력값을 저장할 디바이스 번호 | INT/DINT | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- LIMIT
- S1으로 지정된 입력값이 상하한으로 지정한 범위의 값 여부에 따라 제어된 출력값이 D로 저장됩니다.
- 출력 조건:

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 출력값의 상한이 출력값의 하한보다 작을 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| LIMIT(P) DLIMIT(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DLIMIT source figure 1](assets/60976cb519d90a17-NewItem3303.png)

![DLIMIT source figure 2](assets/2a96406c9639d933-NewItem3305.png)

![DLIMIT source figure 3](assets/df012e9f233528df-NewItem3304.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4281limitlimitpdlimitdlimitp.htm` — §4.28.1; source SHA-256 `89e6a39f8f2f810d62a41275d0b881a498c38a1076bae09736255035727831e7`.
- Additional behavior/operand evidence: `4281limitlimitpdlimitdlimitp.htm`, `245.htm`.

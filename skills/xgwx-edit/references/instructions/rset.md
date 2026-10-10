---
name: "xgk-rset"
description: "Use RSET with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4361rsetrsetp.htm"]
---

# RSET

Manual section(s): 4.36.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `RSET`.

Execution form: the source explicitly pairs this command as `(P)`; `RSET` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `RSET`; the catalog records opcode `354` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전환할 블록 NO. 또는 전환할 블록 NO.가 저장된 디바이스 번호 ( 0~1 ) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- RSET (R_No.set)
- 설정된 블록 번호를 S로 지정된 블록 번호로 전환합니다. 현재 설정된 블록번호는 F158을 읽어 보면 알 수 있습니다.
- STOP상태에서 RUN으로 전환할 경우, 블록 번호는 0으로 초기화 됩니다.
- S값이 최대 블록번호를 넘어갈 경우 에러 플래그(F110)를 셋(Set)합니다.
- 전환 가능한 블록 번호는 다음과 같습니다.
- XGK-CPUH/XGK-CPUA : 0~1
- XGK-CPUS/XGK-CPUE : 0
- XGK-CPUS의 경우, RSET명령어는 사용하지 않아도 됩니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| RSET(P) | 2 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![RSET source figure 1](assets/f503e382548bbdd2-NewItem3439.png)

![RSET source figure 2](assets/070b8bd529723bab-NewItem60.png)

![RSET source figure 3](assets/25073684a11bc4fa-NewItem200.png)

![RSET source figure 4](assets/25073684a11bc4fa-NewItem199.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4361rsetrsetp.htm` — §4.36.1; source SHA-256 `e1d4bb6e44eb1298892208f67bf6d063388a75ada079014a023b107cb06bc5ba`.
- Additional behavior/operand evidence: `4361rsetrsetp.htm`, `245.htm`.

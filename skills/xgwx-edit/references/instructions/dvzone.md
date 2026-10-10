---
name: "xgk-dvzone"
description: "Use DVZONE with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4284vzonevzonepdvzonedvzonep.htm"]
---

# DVZONE

Manual section(s): 4.28.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DVZONE`.

Execution form: the source explicitly pairs this command as `(P)`; `DVZONE` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DVZONE`; the catalog records opcode `337` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | 제어하고자 하는 입력값이 있는 디바이스 번호 | INT/DINT | PMK, L, Z, U, N, D, R | Not allowed |
| 2 | S2 | Vertical Zone의 세로 반경 | INT | PMK, L, T, C, Z, U, N, D, R | Allowed |
| 3 | S3 | Vertical Zone의 기울기 역수[%], (0%=inf, 100%=1) | INT | PMK, L, T, C, Z, U, N, D, R | Allowed |
| 4 | D | 출력값을 저장할 디바이스 번호 | INT/DINT | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- VZONE
- S1으로 지정된 입력값이 Vertical Zone의 가로 반경과 기울기에 따라 아래의 그림과 같이 변환된 출력값이 D로 저장됩니다.
- 출력 조건:

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 기울기의 역수가 0~100[%]를 벗어났을 때 Dead Zone의 가로 반경이 0~32767을 벗어났을 때 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| VZONE(P) DVZONE(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DVZONE source figure 1](assets/f504f180de1cf200-NewItem3317.png)

![DVZONE source figure 2](assets/b71980813635f794-NewItem3319.png)

![DVZONE source figure 3](assets/db9e67a96e208daa-NewItem3318.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4284vzonevzonepdvzonedvzonep.htm` — §4.28.4; source SHA-256 `f5cc7555f6b58b13ccfd0e0e189216288c655e56a7446f3a21c429c020be1127`.
- Additional behavior/operand evidence: `4284vzonevzonepdvzonedvzonep.htm`, `245.htm`.

---
name: "xgk-dabsp"
description: "Use DABSP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4353absabspdabsdabsp.htm"]
---

# DABSP

Manual section(s): 4.35.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DABSP`.

Execution form: the source explicitly pairs this command as `(P)`; `DABSP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `DABSP`; the catalog records opcode `804` and `1` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | D | 절대값 변환을 하고자 하는 영역 | WORD/DWORD | PMK, L, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ABS (Absolute Value)
- D로 지정된 영역의 값을 절대값 변환을 취해 다시 D영역에 저장합니다.
- DABS (Double Absolute Value)
- D, D+1로 지정된 영역의 값을 절대값 변환을 취해 다시 D, D+1영역에 저장합니다.

| 구 분 | 실행 전 | 실행 후 |
| --- | --- | --- |
| 영 역 | D0010 | D0010 |
| 데이터 | -00030(hFFE2) | 00030(h001E) |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ABS(P) DABS(P) | 2 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DABSP source figure 1](assets/bcbe3c60b3ca57e2-NewItem3436.png)

![DABSP source figure 2](assets/37197e1b8d617a90-NewItem3437.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4353absabspdabsdabsp.htm` — §4.35.3; source SHA-256 `dbd5de6911eddc9d4bbcad90ecd4ac6aded361e6d5c3eeef6aafba69d0a2b3bd`.
- Additional behavior/operand evidence: `4353absabspdabsdabsp.htm`, `245.htm`.

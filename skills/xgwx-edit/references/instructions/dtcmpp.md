---
name: "xgk-dtcmpp"
description: "Use DTCMPP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4143tcmptcmppdtcmpdtcmpp.htm"]
---

# DTCMPP

Manual section(s): 4.14.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DTCMPP`.

Execution form: the source explicitly pairs this command as `(P)`; `DTCMPP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DTCMPP`; the catalog records opcode `462` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 비교하게 되는 데이터나 데이터 주소 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 비교하게 되는 데이터 주소 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | D | S1과 S2의 비교 결과를 저장할 영역(1워드) | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- TCMP(Table Compare)
- 비교 데이터로 지정된 S1과 S2로 시작되는 16개의 워드 데이터를 비교하여 D로 지정된 영역의 16개 비트에 출력(같으면 1, 다르면 0)합니다.
- S1은 워드데이터 또는 데이터 주소, S2는 Table 선두영역 주소를 지정합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S2로 지정된 영역 + 15(word/dword)한 값이 해당 디바이스 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| TCMP(P) DTCMP(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DTCMPP source figure 1](assets/e54bfcd08c0278f8-NewItem2786.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4143tcmptcmppdtcmpdtcmpp.htm` — §4.14.3; source SHA-256 `59917103751efab74898b75bf4e4a47432a2834eba4a32eabb5ccbedd820debc`.
- Additional behavior/operand evidence: `4143tcmptcmppdtcmpdtcmpp.htm`, `245.htm`.

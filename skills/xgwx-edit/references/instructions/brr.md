---
name: "xgk-brr"
description: "Use BRR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4189brrbrrp.htm"]
---

# BRR

Manual section(s): 4.18.9. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `BRR`.

Execution form: the source explicitly pairs this command as `(P)`; `BRR` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `BRR`; the catalog records opcode `1468` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 연산을 수행하게 될 데이터의 주소 | BYTE | Unknown | Unknown |
| 2 | D | 연산결과를 저장할 주소 | BYTE | PMK, F, L, T, C, S, D.x, R.x, U | Not allowed |
| 3 | n1 | 우측으로 회전시킬 바이트 수 | WORD | Unknown | Unknown |
| 4 | n2 | 우측으로 회전시킬 횟수 | WORD | Unknown | Unknown |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- BRR (Byte Rotate Right)
- S[0]~S[n1-1]바이트까지의 데이터를 우측으로 n2번 회전하여 그 결과를 D[0]~D[n1-1]에 저장하는 명령어입니다.
- n1이 0일 경우 연산을 수행하지 않습니다.

## Manual applicability and restrictions

- 적용 기종 : XGK(XBC-U, XBM-H/H2/HP)

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BRR(P) | 5~8 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![BRR source figure 1](assets/16ef448d624226f5-NewItem2927.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4189brrbrrp.htm` — §4.18.9; source SHA-256 `98fff3da271d1749d44064abc9d5deeca40f90b5ce651e799c21771b104e3c48`.
- Additional behavior/operand evidence: `4189brrbrrp.htm`, `245.htm`.

---
name: "xgk-gmov"
description: "Copy a group of successive WORDs."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4114gmovgmovp1.htm"]
---

# GMOV

Copy a group of successive WORDs.

Manual section(s): 4.11.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GMOV`.

Execution form: the source explicitly pairs this command as `(P)`; `GMOV` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `GMOV`; the catalog records opcode `153` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Transfer N WORDs beginning at S to N WORDs beginning at D.

## Instruction constraints

- N is a WORD count in 0..65,535. Exceeding a device range sets F110 and leaves the result unprocessed.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전송하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | 그룹으로 전송하고자 하는 개수 (0 ~ 65,535) | WORD | PMK, L, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- GMOV(Group Move)
- S로부터 N개 만큼의 워드데이터를 D로 차례로 전송합니다.
- MOV명령은 1 : 1로 워드를 전송하고, GMOV명령은 N : N 으로 워드를 전송합니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set). 해당 명령어 결과는 처리되지 않음. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GMOV(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GMOV source figure 1](assets/70ec8d2d6ff5a7f6-NewItem2678.png)

![GMOV source figure 2](assets/06bfcfb3a5de6fa2-NewItem2679.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4114gmovgmovp1.htm` — §4.11.4; source SHA-256 `16e9479a2d6c13e350225ca6ff4aec22960ee2b69279e22144dea6d38f77c7f8`.
- Additional behavior/operand evidence: `4114gmovgmovp1.htm`, `245.htm`.

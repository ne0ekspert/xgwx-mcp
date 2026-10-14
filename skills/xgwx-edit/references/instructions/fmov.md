---
name: "xgk-fmov"
description: "Fill successive WORDs with the same source WORD."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4115fmovfmovp1.htm"]
---

# FMOV

Fill successive WORDs with the same source WORD.

Manual section(s): 4.11.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `FMOV`.

Execution form: the source explicitly pairs this command as `(P)`; `FMOV` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `FMOV`; the catalog records opcode `154` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Repeat the value S across N destination WORDs beginning at D.

## Instruction constraints

- N is a WORD count in 0..65,535. Exceeding a device range sets F110 and leaves the result unprocessed.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전송하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |
| 3 | N | 그룹으로 전송하고자 하는 워드 개수 (0 ~ 65,535) | WORD | PMK, L, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 워드데이터 S를 D로 부터 N개 워드수 만큼 차례로 전송합니다.
- 데이터의 특정영역을 초기화할 경우 주로 사용합니다.
- N의 범위가 지정영역을 초과하는 경우는 에러 플래그 (F110)를 셋(Set)하고 처리하지 않습니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set). 해당 명령어 결과는 처리되지 않음. | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| FMOV(P) | 4~6 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![FMOV source figure 1](assets/cf2911d9d0209467-NewItem2682.png)

![FMOV source figure 2](assets/7e806f6d8c2d6ab3-NewItem2683.png)

![FMOV source figure 3](assets/284710d46349a468-NewItem2684.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4115fmovfmovp1.htm` — §4.11.5; source SHA-256 `a1d61e88a2cc1861f9c30a8923e7666a68125eab25dd2b5fa24ea53d7b9d8531`.
- Additional behavior/operand evidence: `4115fmovfmovp1.htm`, `245.htm`.

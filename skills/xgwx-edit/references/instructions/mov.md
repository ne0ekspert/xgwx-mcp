---
name: "xgk-mov"
description: "Copy 16-bit data from S to D."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4111movmovpdmovdmovp1.htm"]
---

# MOV

Copy 16-bit data from S to D.

Manual section(s): 4.11.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `MOV`.

Execution form: the source explicitly pairs this command as `(P)`; `MOV` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `MOV`; the catalog records opcode `148` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- MOV copies one WORD.

## Instruction constraints

- A timer or counter WORD operand refers to its current value, which can be read or changed.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전송하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- MOV(Move)
- S로 지정된 디바이스의 워드 데이터를 D로 전송합니다.
- DMOV(Double Move)
- S+1, S로 지정된 디바이스의 더블 워드 데이터를 D+1, D에 전송합니다.

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리(F112) |
| --- | --- | --- | --- | --- |
| MOV(P) DMOV(P) | 2~5 | - | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- MOV명령의 오퍼랜드로 타이머나 카운터를 사용하면 해당 타이머나 카운터의 현재값(1워드)을 읽어 오거나 변경이 가능합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![MOV source figure 1](assets/968b337e194d96d1-NewItem2656.png)

![MOV source figure 2](assets/fac93c74165e2d3c-NewItem2657.png)

![MOV source figure 3](assets/020d523b4800a583-NewItem2659.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4111movmovpdmovdmovp1.htm` — §4.11.1; source SHA-256 `918672cd7f00f6dd535088761c0a1d519028281fba10e82a6d32d16127f97c2d`.
- Additional behavior/operand evidence: `4111movmovpdmovdmovp1.htm`, `245.htm`.

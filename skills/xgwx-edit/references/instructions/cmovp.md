---
name: "xgk-cmovp"
description: "Use CMOVP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4113cmovcmovpdcmovdcmovp1.htm"]
---

# CMOVP

Manual section(s): 4.11.3. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `CMOVP`.

Execution form: the source explicitly pairs this command as `(P)`; `CMOVP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `CMOVP`; the catalog records opcode `410` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | 전송하고자 하는 데이터 또는 데이터가 들어있는 디바이스 번호 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 전송된 데이터를 저장할 디바이스 번호 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- CMOV(Complement Move)
- 워드데이터 S에 1의 보수를 취하여 그 결과를 D에 전송합니다.
- DMOV(Double Complement Move)
- DCMOV(P)명령은 CMOV(P)명령의 2배의 데이터를 1의 보수를 취해 전송합니다. (더블워드 전송)

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| CMOV(P) DCMOV(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![CMOVP source figure 1](assets/e3abe6ec88073846-NewItem2672.png)

![CMOVP source figure 2](assets/7348dfd09c8cf582-NewItem2674.png)

![CMOVP source figure 3](assets/b33730b32f597372-NewItem2675.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4113cmovcmovpdcmovdcmovp1.htm` — §4.11.3; source SHA-256 `a38f37225629d3ebddaaf993813eba7a6f3877a04a0b6cfc9359b309ddc7f168`.
- Additional behavior/operand evidence: `4113cmovcmovpdcmovdcmovp1.htm`, `245.htm`.

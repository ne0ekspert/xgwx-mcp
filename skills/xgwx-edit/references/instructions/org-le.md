---
name: "xgk-org-le"
description: "Combine the current circuit result with an OR comparison: ORG<=."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["41512orgxordgx.htm"]
---

# ORG<=

Combine the current circuit result with an OR comparison: ORG<=.

Manual section(s): 4.15.12. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ORG<=`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_comparison` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `G<=`; the catalog records opcode `1330` and `3` operands in order.
OR placement requires an inspected supported parallel branch. The insertion tool does not construct the branch automatically; use `edit_ladder_branch` only where its guarded shape supports the requested circuit.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Behavior

- Compare N pairs of 16-bit signed integers; every pair must satisfy S1 <= S2 for the comparison to be On.
- Combine the comparison result with the current bit result (BR) using OR.

## Instruction constraints

- N is a WORD group count. Exceeding a source device range sets F110.

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 비교하게 되는 데이터나 데이터 주소 | INT | PMK, F, L, T, C, Z, U, N, D, R | Not allowed |
| 2 | S2 | S1과 비교하게 되는 데이터나 데이터 주소 | INT | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | N | 그룹 비교할 개수 | WORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- ORG X (G=, G>, G<, G>=, G<=, G<>)
- S1 과 S2를 N개 비교하여 X조건과 모두 일치하면 On, 불일치하면 Off하여 이 결과와 현재의 연산결과를 OR하여 새로운 연산결과로 취한다.
- S1 과 S2의 비교는Signed연산으로 실행합니다.
- h8000(-32,768) ~ hFFFF(-1) < 0 ~ h7FFF(32,767)와 같은 결과를 취하게 됩니다.
- ORDG X (DG=, DG>, DG<, DG>=, DG<=, DG<>)
- h80000000(-2,147,483,648) ~ hFFFFFFFF(-1) < 0 ~ h7FFFFFFF(2,147,483,647)와 같은 결과를 취하게 됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | N의 범위가 지정 영역을 초과할 경우 셋(Set). | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ORG X ORDG X | 4~5 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ORG<= source figure 1](assets/ff8333e64e2f0d69-NewItem2831.png)

![ORG<= source figure 2](assets/8ede80fe670d9f68-NewItem2832.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `41512orgxordgx.htm` — §4.15.12; source SHA-256 `9ea4cf91dd31d407d779df867853263e8ee1e7a82d5a07a268a66819b1b0895c`.
- Additional behavior/operand evidence: `344.htm`, `41510loadgxloaddgx.htm`.

---
name: "xgk-daddb"
description: "Use DADDB with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4211addbaddcpdaddbdaddbp.htm"]
---

# DADDB

Manual section(s): 4.21.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DADDB`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DADDB`; the catalog records opcode `241` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S1 | S2와 BCD덧셈을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | S2 | S1과 BCD덧셈을 실행할 데이터 | WORD/DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 3 | D | 연산결과를 저장할 주소 | WORD/DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- ADDB (BCD ADD)
- BCD데이터 S1, S2를 서로 더하여 그 결과를 D에 저장합니다.
- 연산결과에 따라 에러(F110), 제로(F111), 캐리(F112) 플래그를 셋(Set)합니다.
- DADDB (BCD Double ADD)
- BCD데이터 (S1+1 ,S1), (S2+1, S2)를 서로 더하여 그 결과를 (D+1, D)에 저장합니다.
- S1 과 S2 에 0~99,999,999 ( BCD 8자리)를 지정할 수 있 습니다.
- 99,999,999를 초과한 경우 자리 올림은 무시되고 캐리 플래그는 셋(Set)됩니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | S1과 S2의 값이 BCD형식이 아닐 경우 | F110 |
| 제로 | 연산결과가 제로이면 셋(Set)합니다. | F111 |
| 캐리 | 연산결과가 오버플로우이면 셋(Set)합니다. | F112 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| ADDB(P) DADDU(P) | 4~6 | O | O | O |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DADDB source figure 1](assets/af299a0e682a3efe-NewItem2997.png)

![DADDB source figure 2](assets/659f3efbb8dd4f46-NewItem2998.png)

![DADDB source figure 3](assets/ba0ebed30f69ed72-NewItem2999.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4211addbaddcpdaddbdaddbp.htm` — §4.21.1; source SHA-256 `394d77fed4a793d30058df58a4bcae9d7771e190a102da1af26f831cf7cba0fa`.
- Additional behavior/operand evidence: `4211addbaddcpdaddbdaddbp.htm`.

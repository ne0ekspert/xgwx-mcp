---
name: "xgk-dbinha"
description: "Use DBINHA with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4262binhabinhapdbinhadbinhap.htm"]
---

# DBINHA

Manual section(s): 4.26.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `DBINHA`.

Execution form: the source explicitly pairs this command as `(P)`; `DBINHA` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `DBINHA`; the catalog records opcode `290` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | S | ASCII변환을 실시할 데이터 또는 주소 | DWORD | PMK, F, L, T, C, Z, U, N, D, R | Allowed |
| 2 | D | 연산 결과를 저장하게 되는 주소 | DWORD | PMK, L, T, C, Z, U, N, D, R | Not allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual flags

- BINHA (Binary to Hex ASCII)
- Bin 입력값 16비트를 16진수로 나타냈을 때의 각 자리수를 상위부터 차례로 ASCII값으로 변환합니다.
- ASCII변환 된 값들을 D에서부터 한 워드당 2자리씩 차례로 저장합니다.
- BINHA에서 연산 범위는 h0000부터 hFFFF까지입니다.
- 예를 들면, S 에 h02A6를 지정했을 경우, 이후에는 다음과 같이 저장됩니다.
- DBINHA (Double Binary to Hex ASCII)
- Bin 입력값 32 비트를 16진수로 나타냈을 때의 각 자리수를 상위부터 차례로 ASCII값으로 변환합니다.
- ASCII변환된 값들을 D에서부터 한 워드당 2자리씩 차례로 저장합니다.
- DBINHA에서 연산범위는 h00000000부터 hFFFFFFFF까지입니다.

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | 데이터 테이블의 범위가 해당 디바이스 범위를 벗어날 때 | F110 |
| 제로 | 데이터 테이블에 데이터가 없을 때 | F111 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XGB

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| BINHA(P) DBINHA(P) | 2~4 | - | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![DBINHA source figure 1](assets/0653be58dd12f468-NewItem3178.png)

![DBINHA source figure 2](assets/52996f4628d5d436-NewItem3180.png)

![DBINHA source figure 3](assets/f920bd2720531930-NewItem3179.png)

![DBINHA source figure 4](assets/6e13f19e5900909b-NewItem3181.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4262binhabinhapdbinhadbinhap.htm` — §4.26.2; source SHA-256 `ee601564c70925df186787abff742ad35b6430a43dd9f2f8e780fffed3b28731`.
- Additional behavior/operand evidence: `4262binhabinhapdbinhadbinhap.htm`, `245.htm`.

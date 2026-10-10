---
name: "xgk-putm"
description: "Use PUTM with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4432putmputmp.htm"]
---

# PUTM

Manual section(s): 4.43.2. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `PUTM`.

Execution form: the source explicitly pairs this command as `(P)`; `PUTM` is the non-P partner. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `PUTM`; the catalog records opcode `347` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 모션 모듈이 장착된 슬롯번호 | WORD | Unknown | Unknown |
| 2 | S1 | 모션 모듈의 고정영역 선두 어드레스 | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | S2 | 모션 모듈에 저장하고자 하는 데이터가 저장된 Device 이름 및 번호 | DWORD | PMK, L, Z, N, D, R | Allowed |
| 4 | N | 저장할 데이터의 개수 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 모션 모듈에 데이터를 Write하고자 하는 경우 사용되는 명령어입니다.
- sl(특수 모듈의 슬롯번호)로 지정된 모션 모듈의 메모리(S1으로 지정)에 S2로 지정된 Device로부터 N개만큼의 Double Word 데이터를 Write합니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| PUT/GET 에러 | 1. 지정한 베이스/슬롯에 모듈이 없거나 모션 모듈이 아닌 경우 2. PUT/GET 명령을 제대로 실행하지 못 했을 경우 3. CPU별 지원하는 max base 범위를 넘어선 경우 4. 지정한 슬롯에 장착된 모듈에 S로 지정한 어드레스가 없을 경우 5. ax값이 범위를 벗어날 경우 | F0015~F0022 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- S로 지정한 어드레스부터 N개까지의 영역이 해당 블록을 넘어갈 경우 에러가 발생합니다.
- 이 에러는 현재 XG5000에서 입력할 수 없도록 차단되어 있습니다.
- 지정한 슬롯에 장착된 모듈이 모션 모듈이 아니거나, 장착된 모듈에 S로 지정한 어드레스가 없을 경우 PUT/GET 에러(F0015~F0022)와 연산에러(F110)를 함께 셋(Set)합니다.
- 이는 특수 모듈의 특성에 따라 고정영역 주소가 다를 수 있기 때문입니다.
- 지정한 슬롯에 장착된 모듈이 없는 경우 PUT/GET 에러(F0015~F0022)만 셋(Set)합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| PUTM(P) | 4~7 | O | - | - |

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![PUTM source figure 1](assets/ca3b12ac93b6258f-NewItem4038.png)

![PUTM source figure 2](assets/821c3c819afa0473-NewItem4037.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4432putmputmp.htm` — §4.43.2; source SHA-256 `b3070c92c4b5b599c9ac10d6f702b56b75048618e48e094496ab4689b66b0f94`.
- Additional behavior/operand evidence: `4432putmputmp.htm`, `245.htm`.

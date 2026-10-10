---
name: "xgk-xstc"
description: "Use XSTC with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44262xstc1.htm"]
---

# XSTC

Manual section(s): 4.42.62. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSTC`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XSTC`; the catalog records opcode `1222` and `7` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축(종축):1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 주축 토크비 | WORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 종축 토크비 | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 주축 속도비(XGF-PN8A전용) | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | 종축 속도비(XGF-PN8A전용) | WORD | PMK, L, Z, N, D, R | Allowed |
| 7 | n5 | 주축 설정 (1~8:1축~8축, 9:엔코더1, 10:엔코더2) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈의 해당 축에 토크 동기를 실행하도록 하는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축이 토크동기를 실행하도록 합니다.
- 명령을 실행하는 축은 n5에 설정된 축을 주축으로 토크 동기를 실행 합니다.
- 명령을 실행하는 축은 n1, n2에 설정된 토크비 및 n3, n4에 설정된 속도비로 토크동기 운전을 합니다.
- 종축 토크 = (n2/n1) * 주축토크
- 종축 토크동기 속도 = (n4/n3) * 주축 속도

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면, 에러가 발생(F110이 Set)합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XSTC | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 명령어의 파라미터 n3,n4는 XGF-PN8A 전용 파라미터 입니다.
- 종축 토크동기 속도 = (n4/n3) * 주축 속도
- 이 동작은 여러 스캔만에 완료될 수 있습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSTC source figure 1](assets/77fbddcd4368d22c-NewItem3994.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44262xstc1.htm` — §4.42.62; source SHA-256 `65ace6d0ee9448ab9ef2316c138c3272602a8f8edc6deb269cf3c44d0a7eddb0`.
- Additional behavior/operand evidence: `44262xstc1.htm`.

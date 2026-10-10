---
name: "xgk-xswr"
description: "Use XSWR with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44244xswr.htm"]
---

# XSWR

Manual section(s): 4.42.44. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSWR`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XSWR`; the catalog records opcode `1168` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | S | Not unambiguously specified in this topic | WORD | PMK, L, Z, N, D, R | Not allowed |
| 4 | n1 | 모듈 내부 메모리 데이터 읽기 선두 번지 (0 ~ 12147) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈이 동시기동 할 때 필요한 스텝 데이터를 설정하는 명령어 입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 사용할 동시기동 운전 스텝 값들을 S로 지정된 CPU영역에서 n1개수만큼 위치결정 모듈로 옮깁니다.
- 이 명령 실행 수 XSST명령을 실행하면 해당 축들의 동시기동을 실행할 수 있습니다.
- n1에 설정되는 포인트 운전 스텝 개수는 1이상 4이하의 범위입니다.
- S로 지정된 CPU영역에서 읽어오는 값은 아래와 같습니다.

| CPU 영역 | Size | 포인트 운전 스텝 |
| --- | --- | --- |
| S | WORD | 1번축 동시기동 운전 스텝 |
| ~ |  | ~ |
| S+3 | WORD | 4번축 동시기동 운전 스텝 |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XSWR | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 동시기동 명령(XSST)이 실행되기 전까지 축별 동시기동 스텝 설정이 완료(XSWR) 되어야 합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSWR source figure 1](assets/6ddbfff86b1fd46d-NewItem3926.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44244xswr.htm` — §4.42.44; source SHA-256 `816b24e7d16b75fd396660ad91ce93e28ff6afb98b7c582df1963366f808f005`.
- Additional behavior/operand evidence: `44244xswr.htm`.

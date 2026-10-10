---
name: "xgk-xvrd"
description: "Use XVRD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44245xvrd.htm"]
---

# XVRD

Manual section(s): 4.42.45. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XVRD`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XVRD`; the catalog records opcode `1198` and `6` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1(1축) ~ 8(8축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 모듈 내부 메모리 데이터 읽기 선두 번지 | DWORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 블록 간 옵셋 (0 ~ 54217) * XGF-PNxB, XBF-PN0xB :0~65535 | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 한 블록의 크기 (1 ~ 128) | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | 읽을 블록의 개수 (1 ~ 128) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 해당 지령은 파라미터, 운전데이터, 캠데이터 중 위치결정 모듈 내부 메모리의 n1에 설정된 “읽기 주소”위치에서 부터 n3에 설정된 “블록 크기”만큼의 데이터를 WORD 단위로 CPU로 읽어오는 명령입니다.
- n4에 설정된 “블록 개수”가 2 이상인 경우 “읽기 주소”에서부터 n2에 설정된“블록 옵셋”만큼 떨어져 있는 블록들을 “블록 개수”-1 횟수만큼 차례로 읽어옵니다.
- 한 명령으로 읽을 수 있는 최대 데이터 크기 (블록 크기 x 블록 개수)는 128 WORD 입니다.
- “가변 데이터 읽기” 명령은 운전 중에도 실행할 수 있습니다.

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
| XVRD | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 가변 데이터 읽기 명령 실행 후 최소 2ms 후에 GETM 명령을 실행해야 읽은 데이터가 데이터 공용 영역에 저장됩니다
- 다음과 같은 경우에 에러가 되어 “가변 데이터 읽기” 명령을 실행할 수 없습니다.
- 데이터 설정 에러 (에러코드 :711)
- (1) 읽을 데이터 크기(블록 크기 x 블록 개수)가 0이거나 128 WORD 를 초과한 경우
- (2) 읽을 데이터 주소 [읽기 주소 + {블록 옵셋 x (블록 개수 - 1)} + 블록 크기]가 마지막 번지 값(54217)을 초과한 경우
- XGK PLC에서 가변 데이터 읽기 명령을 실행하면, 위치결정 모듈에서 읽은 데이터는 데이터 공용 영역에 저장됩니다.
- PLC 프로그램에서 사용가능도록 디바이스에 저장하기 위해서는GETM 명령[읽을 주소 :0, 데이터 크기 :읽은 데이터 크기(DWORD)]을 반드시 사용해야 합니다.
- XGI/XGR PLC에서는 펑션블록에 설정한 레지스터에 읽은 데이터가 자동으로 저장됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XVRD source figure 1](assets/e7d4cd831a07c092-NewItem3929.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44245xvrd.htm` — §4.42.45; source SHA-256 `a2ff640198a90b61806b418a587d3b9536ef5366a32b1f9c7ea28e6090987a7a`.
- Additional behavior/operand evidence: `44245xvrd.htm`.

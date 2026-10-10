---
name: "xgk-xsbp"
description: "Use XSBP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44230xsbp.htm"]
---

# XSBP

Manual section(s): 4.42.30. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSBP`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XSBP`; the catalog records opcode `1178` and `5` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 기본 파라미터 티칭을 실행할 지령 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭 데이터 (기본파라미터 중 변경할 항목의 변경값) | DWORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 기본 파라미터 중 변경할 항목 (1~21, 255(hFF)) | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 티칭 방법 설정 (0:RAM티칭 혹은 1:ROM티칭) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 해당 지령은 지정 축의 n2(기본 파라미터 항목 중 변경할 항목)의 값을 n1(설정 값)으로 변경하는 명령입니다.
- n3항목을 0:RAM설정을 한 경우에는 변경한 값이 위치결정 모듈에 전원을 인가한 동안만 유지되며, 1:ROM설정을 한경우에는 위치결정 모듈에 전원을 인가하지 않았을 때도 유지가 됩니다.
- 위치결정 모듈의 기본 파라미터는 FRAM에 저장되므로 ROM설정의 실행 횟수는 제한이 없습니다.
- 기본 파라미터의 전체 항목을 한번의 XSBP 명령으로 모두 설정하려면 n2 파라미터 항목에 hFF(255)값을 설정해야 합니다. 기본 파라미터의 항목들은 아래 복수 티칭 메모리 번지에 먼저 저장되어 있어야 합니다.
- 데이터는 PUT 명령 또는 XTWR 명령으로 설정할 수 있습니다.
- 기본 파라미터 설정 항목은 각 제품별로 차이가 있으니 각 제품 사용설명서를 참조하여 주십시오.
- PUT명령을 사용할 때 메모리 번지는 각 제품 사용설명서를 참조하여 주십시오.
- 명령어집에 위치 결정 모듈 기종 별 모든 항목을 표시하는 데에는 한계가 있어, 네트워크 위치결정 모듈을 기준으로 작성된 점을 참고해 주시기 바랍니다.
- 기본 파라미터 설정 항목 (XGF-PN4B/8B 또는 XBF-PN04B/08B 기준)

| 파라미터 항목 | 파라미터 항목 | 파라미터 값 설정 범위 |
| --- | --- | --- |
| 1 | 속도 제한치 | ㎜ :1 ∼ 2,147,483,647 [X10-2㎜/분] Inch :1 ∼ 2,147,483,647 [X10-3Inch/분] degree :1 ∼ 2,147,483,647 [X10-3 degree/분] pulse :1 ∼ 2,147,483,647 [pulse/초] |
| 2 | 가속시간 1 | 1 ~ 2,147,483,647 [ms] |
| 3 | 가속시간 2 | 1 ~ 2,147,483,647 [ms] |
| 4 | 가속시간 3 | 1 ~ 2,147,483,647 [ms] |
| 5 | 가속시간 4 | 1 ~ 2,147,483,647 [ms] |
| 6 | 감속시간 1 | 1 ~ 2,147,483,647 [ms] |
| 7 | 감속시간 2 | 1 ~ 2,147,483,647 [ms] |
| 8 | 감속시간 3 | 1 ~ 2,147,483,647 [ms] |
| 9 | 감속시간 4 | 1 ~ 2,147,483,647 [ms] |
| 10 | 급정지 감속시간 | 1 ~ 2,147,483,647 [ms] |
| 11 | 1회전당 분주 출력 펄스수 | 1 ~ 200,000,000 |
| 12 | 1회전당 이송거리 | 1 ~ 200,000,000 |
| 13 | 단위 | 0:Pulse, 1:mm, 2:Inch, 3:Degree |
| 14 | 단위배정도 | 0:x 1, 1:x 10, 2:x 100, 3:x 1000 |
| 15 | 속도 명령 단위 | 0:단위/시간, 1:rpm |
| 16 | 엔코더 선택*1 | 0:인크리멘탈 엔코더, 1:앱설루트 엔코더 |
| 17 | 현재 위치 표시 보정량 | 0 ~ 255 |
| 18 | 사용자 지정 위치 표시 배율 | 0 ~ 7 |
| 19 | 사용자 지정 속도 표시 배율 | 0 ~ 7 |
| 20 | 축 타입 | 0:실축, 1:가상축 |
| 21 | 토크명령단위 | 0:1%, 1:0.1% |

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
| XSBP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 복수 티칭 데이터 개수에 따라 여러 스캔이 소요될 수 있습니다.
- 티칭 완료 플래그는 존재하지 않습니다.
- 복수의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSBP source figure 1](assets/b73d02cef5027344-NewItem3886.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44230xsbp.htm` — §4.42.30; source SHA-256 `095a0d975daf71f39a7e8dc0bcfbd0498dcf98973261177ac288fff6dd6f4773`.
- Additional behavior/operand evidence: `44230xsbp.htm`.

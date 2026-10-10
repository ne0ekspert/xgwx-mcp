---
name: "xgk-tbp"
description: "Use TBP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44133tbp1.htm"]
---

# TBP

Manual section(s): 4.41.33. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TBP`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `TBP`; the catalog records opcode `1004` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭데이터 (기본파라미터 중 변경할 항목의 변경값) | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 기본파라미터 중 변경할 항목 (1~16), hFF | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 기본 파라미터 티칭 지령을 내리는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축의 기본파라미터 중 n2의 항목을 n1값으로 변경하는 기본 파라미터 티칭 명령을 합니다..
- 파라미터 항목에 설정할 수 있는 값은 아래와 같습니다.
- 기본 파라미터의 전체 항목을 한번의 TBP 명령으로 모두 설정하려면 n2에 hFF(255)값을 설정해야 합니다.

| 항목 | 항목 | 설정값 |
| --- | --- | --- |
| 1 | 속도 제한치 | LD:1~1000000 OC:1~200000 |
| 2 | 바이어스 속도 | LD:1~1000000 OC:1~200000 |
| 3 | 가감속 시간 1 | 0~65535 |
| 4 | 가감속 시간 2 | 0~65535 |
| 5 | 가감속 시간 3 | 0~65535 |
| 6 | 가감속 시간 4 | 0~65535 |
| 7 | 1회전당 펄스수 | 1~65535 |
| 8 | 1회전당 이송 거리 | 1~65535 |
| 9 | 펄스 출력 모드 | 0:CW/CCW 1:PLS/DIR 2:Phase A/B |
| 10 | 단위 | 0:pulse 1:mm 2:inch 3:degree |
| 11 | 단위 배정도 | 0:x1 1:x10 2:x100 3:x1000 |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| TBP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 기본 파라미터 티칭 명령으로 수정한 파라미터 값은 전원이 ON 되어 있는 동안에만 유효합니다.
- 기본 파라미터 티칭 명령으로 수정한 파라미터 값을 플래시 메모리에 저장하려면 기본 파라미터 티칭 후 파라미터/운전데이터 저장 명령 WRT 를 사용하여 수정한 파라미터 값을 플래시 메모리에 저장해야 합니다.
- 복수 티칭 데이터 개수에 따라 여러 스캔이 소요될 수 있습니다.
- 티칭 완료 플래그는 존재하지 않습니다. 따라서 복수개의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해 주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TBP source figure 1](assets/7b07addfa47ee450-NewItem3719.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44133tbp1.htm` — §4.41.33; source SHA-256 `742da807dfaad90efc778a549864f2990e53109c495e26fcf53498f10f813aaa`.
- Additional behavior/operand evidence: `44133tbp1.htm`.

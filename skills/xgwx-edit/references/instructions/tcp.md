---
name: "xgk-tcp"
description: "Use TCP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44138tcp.htm"]
---

# TCP

Manual section(s): 4.41.38. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `TCP`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `TCP`; the catalog records opcode `1014` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 기본 파라미터 티칭을 실행할 지령 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 티칭 데이터(공통 파라미터 중 변경할 항목의 변경값) | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 공통 파라미터 중 변경할 항목 (1~14), hFF | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 공통 파라미터 티칭 지령을 내리는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 공통 파라미터 중 n2의 항목을 n1값으로 변경하는 공통 파라미터 티칭 명령을 합니다..
- n2값은 아래와 같이 설정할 수 있습니다.

| 설정값 | 항 목 | 항 목 |
| --- | --- | --- |
| 1 | 펄스 출력 레벨 | 0:Low Active 1:High Active |
| 2 | 원호보간 방식 | 0:중간점 1:중심점 |
| 3 | 엔코더 펄스 입력모드 | 0:CW/CCW(1상1체배) 1:CW/CCW(1상2체배) 2:Pulse/Dir(1상1체배) 3:Pulse/Dir(1상2체배) 4:PhaseA/B(2상1체배) 5:PhaseA/B(2상2체배) 6:PhaseA/B(2상4체배) |
| 4 | 엔코더 Auto Reload값 |  |
| 5 | Zone출력 모드 | 0:개별출력 1:일괄출력 |
| 6 | Zone1축설정 | 0:X축 1:Y축 2:Z축 3:Encoder |
| 7 | Zone2축설정 | 0:X축 1:Y축 2:Z축 3:Encoder |
| 8 | Zone3축설정 | 0:X축 1:Y축 2:Z축 3:Encoder |
| 9 | Zone1 On영역 |  |
| 10 | Zone1 Off영역 |  |
| 11 | Zone2 On영역 |  |
| 12 | Zone2Off영역 |  |
| 13 | Zone3 On영역 |  |
| 14 | Zone3 Off영역 |  |

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
| TCP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 공통 파라미터 티칭 명령으로 수정한 파라미터 값은 전원이 ON 되어 있는 동안에만 유효합니다.
- 공통 파라미터 티칭 명령으로 수정한 파라미터 값을 플래시 메모리에 저장하려면 공통 파라미터 티칭 후 파라미터/운전데이터 저장 명령 WRT 를 사용하여 수정한 파라미터 값을 플래시 메모리에 저장해야 합니다.
- 복수 티칭 데이터 개수에 따라 여러 스캔이 소요될 수 있습니다.
- 티칭 완료 플래그는 존재하지 않습니다. 따라서 복수개의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해 주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![TCP source figure 1](assets/fca719af379372bb-NewItem4319.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44138tcp.htm` — §4.41.38; source SHA-256 `89c4c023c1ee99a226fe83ecb3a944a4a9f7eb44d20709001b6dddcfc3ca2d6d`.
- Additional behavior/operand evidence: `44138tcp.htm`.

---
name: "xgk-xelin"
description: "Use XELIN with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44240xelin.htm"]
---

# XELIN

Manual section(s): 4.42.40. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XELIN`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XELIN`; the catalog records opcode `1174` and `5` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 타원 보간을 실행 할 지령 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 타원 보간을 실행 할 스텝번호 | WORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 타원의 비율 (1 = 0.01%) (0 ~ 65535) | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 타원 보간을 할 각도(degree) (1 = 0.1°) (0 ~ 65535) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- ax로 지정된 축에 운전데이터 n1스텝을 n2로 설정된 비율로 n3에 설정된 각도만큼 타원 보간하라는 명령을 합니다.
- 2개의 축을 사용해서 명령축(주축)의 데이터에 맞추어 종축을 제어하면서, 목표 위치까지 설정한 비율의 타원의 궤적으로 설정한 운전각도 만큼 위치결정 제어를 실행합니다.
- 타원 보간 제어에 사용할 축의 조합에는 제한이 없으며 1축 ~ 8축 중 두 축이 사용됩니다.
- 타원 보간은 타원의 특성상 반드시 시작 위치와 목표 위치를 같게 설정해야 합니다.
- 타원 보간은 운전 데이터에 중심점 지정 방식의 원호 보간으로 설정하고, 「타원 보간」명령의 보조 데이터로 타원의 비율과 크기를 설정합니다.
- 타원 보간은 설정된 타원 비율에 따라 반지름의 크기가 계속 변하므로 합성속도도 따라서 변화합니다.
- 타원의 이동 방향은 운전 데이터의 “원호 보간 모드”에 설정된 방향으로 결정됩니다.
- (1) 「중심점, CW」 - 현재 위치에서 시계 방향으로 타원 보간을 실행합니다.
- (2) 「중심점, CCW」- 현재 위치에서 반시계 방향으로 타원 보간을 실행합니다.

| 보조 데이터 | 설정값 | 내용 |
| --- | --- | --- |
| 타원의 비율 (%) | 0 ~ 65535 | 정원에 대한 비율로 타원의 가로축과 세로축의 비율을 설정합니다. (65535 = 655.35%) |
| 타원의 크기(각도) | 0 ~ 65535 | 타원의 이동 각도를 설정합니다. (3650 = 360.0°) |

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
| XELIN | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 타원 보간 제어는 주축(명령축)의 운전 데이터에 설정된 항목을 기준으로 운전을 실행합니다.
- 타원 보간 운전 시 종축의 설정 중 「목표 위치」와 「원호보간 보조점」 이외의 항목은 타원 보간 운전에 영향을 미치지 않습니다.
- 어떤 값을 설정하더라도 동작에 영향을 미치지 않고, 에러도 발생하지 않습니다
- 타원의 비율이 100% 이상일 때는 종축의 속도가 빨라져서 합성 속도가 빨라지므로 운전에 주의가 필요합니다.
- 타원 보간 종축에 대해서는 「속도 제한치」로 속도를 제한하지 않으므로 종축의 속도가 속도 제한치를 넘지 않도록 운전 속도를 설정하시기 바랍니다.
- 다음과 같은 경우에 에러가 되어 타원 보간 제어를 실행할 수 없습니다.
- 「종축설정」이상 (에러코드 :547)
- (1) 주축 운전 데이터의 「종축설정」 값이 “축미설정”인 경우
- (2) 주축 운전 데이터의 「종축설정」 값을 주축 번호와 같게 설정한 경우
- (3) 주축 운전 데이터의 「종축설정」 값이 현재 사용중인 모듈의 설정 가능한 축 번호 초과인 경우
- (4) 헬리컬 보간 축을 설정한 경우
- 주축 또는 종축의 제어 단위를 “degree”로 설정한 경우 (에러코드 :551(주축), 552(종축))
- 보조점으로 지정한 중심점이 시작 위치 또는 목표 위치와 같은 경우 (에러코드 :553)
- 계산된 원호의 반지름이 2147483647pls를 초과하는 경우 (에러코드 :554)
- 운전 방식을 계속 또는 연속으로 설정한 경우 (에러코드 :556) 타원 보간을 사용할 경우는 반드시 종료 운전으로 설정하고 사용하십시오.
- 시작 위치와 목표 위치가 다른 경우 (에러코드 :558)
- 타원의 크기(이동 각도)가 0인 경우 (에러코드 :559)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XELIN source figure 1](assets/b80433ef1614ffba-NewItem3914.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44240xelin.htm` — §4.42.40; source SHA-256 `0cea06c47e0f3ef20112815265a48ddb511e233828b0a986d9b3ce5941573165`.
- Additional behavior/operand evidence: `44240xelin.htm`.

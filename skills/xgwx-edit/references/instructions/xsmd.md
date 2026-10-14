---
name: "xgk-xsmd"
description: "Use XSMD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44236xsmd.htm"]
---

# XSMD

Manual section(s): 4.42.36. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSMD`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XSMD`; the catalog records opcode `1190` and `6` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 변경할 운전 데이터 값 | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 변경할 운전 데이터 항목(1 ~ 17, 255) | WORD | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 변경할 운전 데이터 스텝(0 ~ 400) | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | 0:RAM설정, 1:ROM설정 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 운전 데이터 항목 중 특정 스텝의 항목 값을 설정 한 값으로 티칭하는 명령입니다
- 해당 지령은 지정 축의 n3(지정 스텝)의 항목 중 n2(운전 데이터 중 변경할 항목)의 값을 n1(설정 값)으로 변경하는 명령입니다
- n4항목을 0:RAM설정을 한 경우에는 변경한 값이 위치결정 모듈에 전원을 인가한 동안만 유지되며, 1:ROM설정을 한경우에는 위치결정 모듈에 전원을 인가하지 않았을 때도 유지가 됩니다.
- 위치결정 모듈의 기본 파라미터는 FRAM에 저장되므로 ROM설정의 실행 횟수는 제한이 없습니다.
- 운전 데이터 항목은 아래와 같습니다.

| 설정값 | 항목 | 설정범위 | 초기값 |
| --- | --- | --- | --- |
| 1 | 목표위치 | ㎜ :-2147483648 ∼ 2147483647 [X10-4㎜] Inch :-2147483648 ∼ 2147483647 [X10-5Inch] degree :-2147483648 ∼ 2147483647 [X10-5degree] pulse :-2147483648 ∼ 2147483647 [pulse] | 0 |
| 2 | 원호보간 보조점 | ㎜ :-2147483648 ∼ 2147483647 [X10-4㎜] Inch :-2147483648 ∼ 2147483647 [X10-5Inch] degree :-2147483648 ∼ 2147483647 [X10-5degree] pulse :-2147483648 ∼ 2147483647 [pulse] | 0 |
| 3 | 운전속도 | ㎜ :1 ∼ 2,147,483,647 [X10-2㎜/분] Inch :1 ∼ 2,147,483,647 [X10-3Inch/분] degree :1 ∼ 2,147,483,647 [X10-3 degree/분] pulse :1 ∼ 2,147,483,647 [pulse/초] | 0 |
| 4 | 드웰시간 | 0 ~ 65,535[ms] | 0 |
| 5 | M코드 번호 | 0 ~ 65,535 | 0 |
| 6 | 종축설정 | Bit 단위 설정 Bit 7 Bit 6 Bit 5 Bit 4 Bit 3 Bit 2 Bit 1 Bit 0 축8 축7 축6 축5 축4 축3 축2 축1 | 축미설정 |
| 7 | 헬리컬보간 축 | 0, 1축 ~ 4축 (0으로 설정 시 일반 원호보간임) | 사용안함 |
| 8 | 원호보간 턴 수 | 0~65,535 | 0 |
| 9 | 좌표 | 0:절대, 1:상대 | 0:절대 |
| 10 | 제어방식 | 0:단축위치제어, 1:단축속도제어, 2:단축Feed제어, 3:직선보간, 4:원호보간 | 0:단축위치제어 |
| 11 | 운전방식 | 0:단독, 1:반복 | 0:단독 |
| 12 | 운전패턴 | 0:종료, 1:계속, 2:연속 | 0:종료 |
| 13 | 원호크기 | 0:원호<180 1:원호>=180 | 0:원호<180 |
| 14 | 가속번호 | 1 ~ 4 | 1 |
| 15 | 감속번호 | 1 ~ 4 | 1 |
| 16 | 원호보간 방법 | 0:중간점, 1:중심점, 2:반지름 | 0:중간점 |
| 17 | 원호보간 방향 | 0:CW, 1:CCW | 0:CW |

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면 에러가 발생(F110이 Set)됩니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XSMD | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 공통 파라미터의 전체 항목을 한번의 XSMD 명령으로 모두 설정하려면 n2 파라미터 항목에 hFF(255)를 적용 할 수 있습니다.
- 복수 티칭에 대한 예제는 XSEP명령어를 참고하시기 바랍니다.
- 운전 데이터 설정 명령은 해당 축이 운전 중일 경우에도 실행 가능합니다.
- 현재 운전 중인 스텝의 운전데이터를 변경하는 경우에는 현재 스텝의 운전이 종료된 후 반영됩니다.
- 티칭 완료 플래그는 존재하지 않습니다.
- 복수의 티칭을 실행하고 티칭 개수에 따라 약간의 딜레이를 주고 다음 지령을 실행 해주시기 바랍니다. (항목 개수에 따라 수백 ms이상)

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSMD source figure 1](assets/0287803890f25d46-NewItem3902.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44236xsmd.htm` — §4.42.36; source SHA-256 `0f6bde6428c51eb0385649511bf217821efd6bcf27cbc2c6de3bcbce91284742`.
- Additional behavior/operand evidence: `44236xsmd.htm`.

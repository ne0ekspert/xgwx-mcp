---
name: "xgk-xsesex"
description: "Use XSESEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44234xses.htm"]
---

# XSESEX

Manual section(s): 4.42.34. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSESEX`.

## Editing route

No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.

## Operands

| 오퍼랜드 | 내 용 | 데이터 크기 |
| --- | --- | --- |
| sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD |
| ax | 기본 파라미터 티칭을 실행할 지령 축:1~4(실축) | WORD |
| n1 | 티칭 데이터 (입/출력 파라미터 중 변경할 항목의 변경값) | WORD |
| n2 | 티칭 방법 설정 (0:RAM티칭 혹은 1:ROM티칭) | WORD |

This source table is not a separately reviewed writer rule. Retain shared variant distinctions and let the current writer reject unsupported operands.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxH를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 지령 축의 입/출력신호 파라미터를 설정된 값(Bit제어)으로 티칭하는 명령입니다.
- 해당 지령은 지정 축의 n2(입출력 파라미터 항목 중 변경할 항목)의 값을 비트 단위로 on/off설정 합니다.
- n2항목을 0:RAM설정을 한 경우에는 변경한 값이 위치결정 모듈에 전원을 인가한 동안만 유지되며, 1:ROM설정을 한경우에는 위치결정 모듈에 전원을 인가하지 않았을 때도 유지가 됩니다.
- 위치결정 모듈의 기본 파라미터는 FRAM에 저장되므로 ROM설정의 실행 횟수는 제한이 없습니다.
- 입/출력신호 파라미터 값의 각 Bit별 해당 신호는 아래와 같습니다.
- 각 Bit가 Set되면 “B접점”, Clear되면 “A접점”으로 동작합니다.

| Bit | 신호 | 설정범위 | 초기값 |
| --- | --- | --- | --- |
| 0 | 상한 신호 | 0:A접점, 1:B접점 | A접점 |
| 1 | 하한 신호 | 0:A접점, 1:B접점 | A접점 |
| 2 | 근사원점 신호 | 0:A접점, 1:B접점 | A접점 |
| 3 | 원점 신호 | 0:A접점, 1:B접점 | A접점 |
| 4 | 비상정지/감속정지 신호 | 0:A접점, 1:B접점 | A접점 |
| 5 | 속도/위치 제어전환 신호 | 0:A접점, 1:B접점 | A접점 |
| 6 | 드라이브 레디 신호 | 0:A접점, 1:B접점 | A접점 |
| 7 | 인포지션 신호 | 0:A접점, 1:B접점 | A접점 |
| 8 | 편차카운터 클리어/설정 위치 출력 신호 | 0:A접점, 1:B접점 | A접점 |
| 9~11 | 입력 필터 | 0:적용안함, 1:1ms, 2:3ms,3:5ms, 4:10ms, 5:20ms,6:50ms, 7:100ms | 0:적용안함 |
| 12 ~ 15 | 사용안함 | 사용안함 | 사용안함 |

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
| XSES | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 티칭 완료 플래그는 존재하지 않습니다.
- 입/출력신호 파라미터 설정 명령은 해당 축이 운전 중일 경우에는 실행할 수 없습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSESEX source figure 1](assets/7a97ac33bc82ec9c-NewItem3898.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44234xses.htm` — §4.42.34; source SHA-256 `c53e27892350143895ddc30b143448810455cf7af181caf2a75b3e267b4df384`.

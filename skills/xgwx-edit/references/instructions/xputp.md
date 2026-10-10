---
name: "xgk-xputp"
description: "Write XGT network-positioning parameters directly."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4432putmputmp1.htm"]
---

# XPUTP

Write XGT network-positioning parameters directly.

Manual section(s): 4.43.5. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XPUTP`.

## Editing route

No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.

## Behavior

- Write N WORDs from S2 to module address S1; sl selects h00xy. A write during motion takes effect after that motion ends.
- Invalid parameter values can be stored but cause a parameter error that blocks a start until corrected.
- This P-suffix variant executes once when the enabling input changes from Off to On.

## Instruction constraints

- Use section 5.1.4 direct parameter memory addresses, not the different variable-data appendix map. Confirm the listed module/CPU/XG5000 versions.

## Source corrections

- The error section mentions XGET/S/ax and the ST example uses S,D names, copied from the read page; the operand table defines XPUT sl,S1,S2,N.

## Operands

| 오퍼랜드 | 내 용 | 데이터 크기 |
| --- | --- | --- |
| sl | 위치결정 모듈이 장착된 베이스 및 슬롯 번호 | WORD |
| S1 | 데이터를 Write할 위치결정 모듈 내부 메모리의 선두 번지 | WORD |
| S2 | Write할 데이터가 저장되어 있는 디바이스의 선두 번지 | WORD |
| N | Write할 데이터의 개수 | WORD |

This source table is not a separately reviewed writer rule. Retain shared variant distinctions and let the current writer reject unsupported operands.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- 이 명령은 위치결정 모듈의 파라미터(기본파라미터, 확장파라미터, 수동운전파라미터, 공통 파라미터)를 직접 쓰기하는 위치결정 모듈(XGF-PN4B/XGF-PN8B) 전용 명령입니다.
- 참고) 명령어의 S 입력값(위치결정 모듈 내부 메모리의 선두 번지)은 XGF-PN4B/XGF-PN8B사용설명서의 5.1.4 파라미터 적접 읽기/쓰기 메모리 번지를 기준으로 설정해야 합니다.
- sl의 설정 방식은 16진수 2자리로 설정하고 아래의 프로그램과 같이 h0013인 경우 첫번째 숫자 ‘1’은 베이스의 번호, 두번째 ‘3’은 슬롯번호를 의미합니다. WORD 단위로 쓰기가 실행됩니다.
- 파라미터 직접 쓰기 명령은 기동 명령과 동시에 실행할 수 있습니다.
- 파라미터 직접 쓰기 명령은 한 스캔에 복수 개 실행이 가능합니다.
- 명령어에 디바이스를 통해 입력된 값이 범위 에러가 있는 경우 값은 해당 메모리에 쓰기는 가능하나 파라미터 에러가 발생하게 됩니다.
- (에러가 제거되기 전에는 기동 명령을 실행할 수 없습니다.)
- 운전 중 파라미터 직접 쓰기가 실행되면 운전이 종료된 후에 반영됩니다.
- 파라미터 직접 읽기/쓰기를 위한 위치결정 모듈 내부 메모리 번지 정보는 XGF-PN4B/8B 사용설명서의‘5.1.4 파라미터 직접 읽기/쓰기’를 참조하시기 바랍니다.
- 주의) 부록 2.1 가변 데이터 읽기/쓰기 명령의 모듈 내부 메모리 주소값이 혼용되지 않도록 주의해 주시기 바랍니다.
- 펑션블록(XGI 타입 CPU)에서는 XPM_GET/XPM_PUT 함수로 파라미터 직접 읽기/쓰기 기능이 제공됩니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| PUT/GET 에러 | 1. 지정한 베이스/슬롯에 모듈이 없거나 모션 모듈이 아닌 경우 2. PUT/GET 명령을 제대로 실행하지 못 했을 경우 3. CPU별 지원하는 max base 범위를 넘어선 경우 4. 지정한 슬롯에 장착된 모듈에 S로 지정한 어드레스가 없을 경우 5. ax값이 범위를 벗어날 경우 (XGF-PN4B: 0~255, 512~521, XGF-PN8B: 0~521) | F0015~F0022 |

## Manual applicability and restrictions

- 적용 기종 : XGK-N
- sl로 지정된 위치에 위치결정 모듈이 없거나 XGET 명령을 정상적으로 수행하지 못 했을 경우 PUT/GET에러 플래그인 XGK F0015~F0022(WORD)의 해당 위치 비트가 셋(Set) 됩니다. 이 때 연산에러 플래그인 F110, F115eh 함께 셋(Set)됩니다.
- sl로 지정된 위치에 모듈이 존재하지 않는 경우에는 PUT/GET 에러 플래그 F0015~F0022 만 셋(Set)되며, 연산에러 플래그 F110, F115는 셋(Set)되지 않습니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XPUT(P) | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 파라미터 적접 읽기/쓰기 메모리 번지 (‘XGF-PN4B/8B 사용설명서 5.1.4 파라미터 직접 읽기/쓰기 요약)
- XGF-PN4B/8B사용설명서의 ‘부록2.1 파라미터 메모리 주소’와 내용이 다릅니다. 번지가 잘못 입력되면 의도하지 않은 파라미터에 값이 입력되므로 주의하시기 바랍니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XPUTP source figure 1](assets/661d94d7fe758106-NewItem4383.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4432putmputmp1.htm` — §4.43.5; source SHA-256 `73780e3d08a5c113087d3f0d3aad696d62de38f30eff5f212fa237a77676c2dd`.
- Additional behavior/operand evidence: `33.htm`.

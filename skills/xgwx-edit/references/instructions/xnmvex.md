---
name: "xgk-xnmvex"
description: "Use XNMVEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44217xnmv.htm"]
---

# XNMVEX

Manual section(s): 4.42.17. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XNMVEX`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XNMVEX`; the catalog records opcode `1701` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 연속 운전 명령을 실행하면 현재 운전중인 스텝 번호가 다음 스텝 번호로 변경되면서 다음 스텝의 속도와 목표 위치로 위치결정 운전을 합니다. 다음 스텝과의 연결은 연속운전 패턴으로 실행됩니다.
- 연속 운전 명령은 현재 실행중인 스텝의 현재 운전 패턴만을 변경하고 운전 데이터를 변경하지는 않습니다.
- 운전 모드가 종료, 계속, 연속 운전일 경우와 운전 패턴 중 가속, 정속, 감속 구간에서 사용할 수 있습니다.

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
| XNMV | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음의 경우는 에러가 되어 연속 운전 명령을 실행하지 않고, 이전의 동작을 계속 합니다.
- 확장 파라미터의 가감속 패턴이 「S-커브운전」인 경우 (에러코드:390)
- 현재 상태가 드웰 중인 경우 (에러코드:392)
- 현재 실행 중인 제어방식이 단축위치제어, 직선보간이 아닌 경우 (에러코드:393)
- 다음에 실행할 운전 스텝의 속도 데이터 값이 0이거나 속도 제한치를 초과하는 경우 (에러코드:394)
- 직선보간의 종축에 대하여 연속 운전 명령을 실행한 경우 (에러코드:395) 직선보간 운전 중인 경우의 연속 운전 명령은 주축에 실행해야 합니다.
- 원호보간 운전 중인 축에 연속 운전 명령을 실행한 경우 (에러코드:396)
- 동기운전 종축에 대하여 연속 운전 명령을 실행한 경우 (에러코드:397)
- 현재 운전 스텝 번호가 운전 데이터의 마지막 스텝(400)인 경우 (에러코드:399)
- 현재 운전 중인 축이 직접 기동으로 실행된 경우 (에러코드:400)
- 위치 데이터의 설정에서 절대 좌표와 상대 좌표에 따라 연속 운전 명령에는 동작의 차이가 있습니다.
- 절대 좌표에서의 연속 운전 명령의 목표 위치는 같으므로, 연속 운전 명령 실행 후의 목표 위치는 연속 운전 명령을 실행하지 않았을 경우와 같습니다.
- 연속 운전 명령에 의해 위치 결정 완료된 현재 위치는 P2가 됩니다. (A영역과 B영역의 크기는 같습니다.)
- 상대 좌표에서의 연속 운전 명령 실행 시는 명령이 입력되는 시점에서의 현재 위치에서 목표 위치에 설정된 이동량 만큼 증분된 값이 목표 위치가 됩니다.
- 연속 운전을 실행하지 않았을 경우와 목표 위치가 달라집니다. 즉, 연속 운전 명령에 의해 위치 결정 완료된 위치는 P1’+ P2 가 됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XNMVEX source figure 1](assets/e13ecf08a3addd27-NewItem3841.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44217xnmv.htm` — §4.42.17; source SHA-256 `8986945e423e2abd2916ab7df0e54ccb8e95cba3c931dcf457b8c52e3fcc0c80`.
- Additional behavior/operand evidence: `44217xnmv.htm`.

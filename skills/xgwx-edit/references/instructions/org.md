---
name: "xgk-org"
description: "Use ORG with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4411org.htm"]
---

# ORG

Manual section(s): 4.41.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `ORG`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `ORG`; the catalog records opcode `948` and `2` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 원점 복귀는 전원을 인가한 후에 기계 원점의 확립을 위해 실행합니다.
- 원점 복귀를 실행하기 전에 각 축마다 원점 복귀 파라미터를 설정해야만 합니다.
- 원점 복귀로 원점 위치가 결정되면 위치결정 운전 중에는 원점 검출 신호를 인식하지 않습니다.

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
| ORG | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 비상정지 발생 시, 축의 원점결정 상태를 ‘0:이전상태 유지’또는 ‘1:원점미결정’중에서 설정합니다.
- 0: 이전상태 유지:비상정지로 인하여 축이 정지하여도 원점결정 상태가 변하지 않습니다.
- 1: 원점미결정:비상정지로 인하여 축이 정지한 경우 원점결정 상태가 해제 됩니다.
- 원점 복귀 속도를 설정할 때에는 원점 복귀 저속 속도는 가능한 낮은 속도로 설정해 주는 것이 좋습니다.
- 원점 복귀 저속 속도를 너무 빠르게 설정하면 원점 신호 검출시 원점 위치와 원점 복귀 완료 후 위치 사이에 오차가 발생할수 있습니다.
- 원점 복귀가 완료된 상태에서 외부 입력 신호(근사원점과 원점신호)는 위치 결정 제어 영향을 주지 않습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![ORG source figure 1](assets/f2d9370213450e34-NewItem3575.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4411org.htm` — §4.41.1; source SHA-256 `ca074319c00d6168fd81f82d61638efd10fcff7e2fedd0fbfa95e423b80817a6`.
- Additional behavior/operand evidence: `4411org.htm`.

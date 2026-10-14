---
name: "xgk-xrtpex"
description: "Use XRTPEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44219xrtp.htm"]
---

# XRTPEX

Manual section(s): 4.42.19. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XRTPEX`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XRTPEX`; the catalog records opcode `1656` and `2` operands in order.
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
- 위치결정 완료 후에 외부축 속도 동기운전, 인칭 운전, 조그 운전으로 현재 위치가 변경되었을 때 수동 운전을 실행하기 이전의 위치로 복귀합니다.
- 수동 운전을 하지 않은 상태에서는 수동 운전 이전 위치 복귀 명령은 무시됩니다.
- 이동 방향은 현재의 정지 위치와 수동 운전 이전 위치에 따라 결정 됩니다.
- (1) 시작 위치 < 수동 운전 이전 위치 :정방향으로 위치결정 운전
- (2) 시작 위치 > 수동 운전 이전 위치 :역방향으로 위치결정 운전

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
| XRTP | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 가감속 처리와 수동 운전 이전 위치 복귀 속도 관련 사항
- 수동 운전 이전 위치 복귀의 가감속 시간은 수동운전 파라미터의 조그 가속/감속시간을 사용합니다.
- XG-PM의 수동운전 파라미터 설정 항목 중 조그 가속/감속시간에 가속과 감속 시간을 설정 하십시오.
- 수동 운전 이전 위치 복귀 속도는 XG-PM의 수동운전 파라미터 설정 항목 중 조그 저속 속도를 사용합니다.
- 수동 운전 이전 위치 복귀 속도는 설정 범위를 초과하여 설정하면 에러가 발생되고 운전이 되지 않습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XRTPEX source figure 1](assets/8d5edf063ae79239-NewItem3848.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44219xrtp.htm` — §4.42.19; source SHA-256 `c59d13d2eb09b4fbe92204c540f17ee2845cc5bb0d9a0bd10b5fe05b4f7c2ea9`.
- Additional behavior/operand evidence: `44219xrtp.htm`.

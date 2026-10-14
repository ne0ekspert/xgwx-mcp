---
name: "xgk-xssspd"
description: "Use XSSSPD with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44265xssspd.htm"]
---

# XSSSPD

Manual section(s): 4.42.65. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSSSPD`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XSSSPD`; the catalog records opcode `1091` and `6` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 직접 기동 지령을 실행할 축:1(1축) ~ 8(8축), 9(엔코더) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 속도동기 주축비 (-2147483648~2147483647) | DINT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 속도동기 종축비 (-2147483648~2147483647) | DINT | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 속도동기 주축설정:1(1축) ~ 8(8축), 9(엔코더) | WORD | PMK, L, Z, N, D, R | Allowed |
| 6 | n4 | 속도동기 목표위치:(-2147483648 ~ 2147483647) | DINT | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 32비트 위치 지정 속도 동기 명령은, 기본적으로 위치 지정 속도 동기 지령과 동일 합니다.
- 다만, 주축비와 종축비의 값을 32비트 정수 범위로 설정할 수 있습니다.
- 32비트 위치 지정 속도 동기 명령을 실행하면 종축은 속도 동기 운전중 상태가 되고, 종축에 정지 명령을 실행하면 종축의 속도 동기 운전중 상태는 해제됩니다.
- 속도 동기 비율을 다음과 같이 계산됩니다. (속도 동기 비율 = 종축 설정값 / 주축 설정값)
- 종축의 운전 방향은 속도 동기 비율의 부호에 따라 결정되며, 양수이면 주축의 운전 방향으로,음수이면 주축의 운전 반대 방향으로 운전됩니다.
- 종축이 속도 동기 운전중 상태에서도 속도 동기 비율을 변경할 수 있습니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| 에러 | ax 값이 범위를 벗어날 경우 | F110 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- ax의 설정값이 제품에서 지원하는 설정 범위와 다른 값을 입력하면, 에러가 발생(F110이 Set)합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| XSSSPD | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- M 코드 신호가 On된 축은, 에러(코드:353)가 발생하여 위치 지정 속도 동기 명령의 실행할 수 없습니다.
- 주축과 종축을 같게 설정하며, 에러(코드:355)가 발생합니다.
- 32비트 위치 지정 속도 동기 비율에 의하여 종축의 속도가 속도 제한치를 벗어나는 경우는 에러(코드:357)이 발생합니다. 종축이 위치 지정 속도 동기 운전중에 속도 제한치를 벗어나는 경우는 기본 파라미터의‘급정지시 감속시간’으로 감속 정지합니다.
- 위치 지정 속도 동기 명령으로 종축의 속도를 변경하는 경우에, 변경 전후의 속도 차이가 크면 장비와 기계에 충격이 발생할 수 있으므로 주의하시기 바랍니다.
- 종축에 위치 지정 속도 동기 명령을 실행하고 주축에 원점 복귀 명령을 실행할 수 있으나, 주축이 원점 복귀 동작중일 때는 종축에 위치 지정 속도 동기 명령을 실행할 수 없습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSSSPD source figure 1](assets/0a8de559d6089c54-NewItem4008.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44265xssspd.htm` — §4.42.65; source SHA-256 `b2e39fd87720c7ac93755ca98aedaae71c2d767d5b5efd60bf2a32cb52d22faa`.
- Additional behavior/operand evidence: `44265xssspd.htm`.

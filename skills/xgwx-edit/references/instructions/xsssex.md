---
name: "xgk-xsssex"
description: "Use XSSSEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44213xsss.htm"]
---

# XSSSEX

Manual section(s): 4.42.13. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSSSEX`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XSSSEX`; the catalog records opcode `1666` and `5` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 속도 동기를 실행할 종축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 속도 동기 비율의 주축 설정값:-32768~32767 | INT | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 속도 동기 비율의 종축 설정값:-32768~32767 | INT | PMK, L, Z, N, D, R | Allowed |
| 5 | n3 | 속도 동기가 실행될 주축:1~10(1~8:실축, 9~10:엔코더1~엔코더2) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 속도 동기 명령은, 설정한 속도 동기 비율로, 주축이 움직이는 패턴과 동일하게, 종축이 움직이도록 합니다.
- 속도 동기 명령을 실행하면 종축은 속도 동기 운전중 상태가 되고, 종축에 정지 명령을 실행하면 종축의 속도 동기 운전중 상태는 해제됩니다.
- 속도 동기 비율을 다음과 같이 계산됩니다. 속도 동기 비율 = 종축 설정값 / 주축 설정값
- 종축의 운전 방향은 속도 동기 비율의 부호에 따라 결정되며, 양수이면 주축의 운전 방향으로,음수이면 주축의 운전 반대 방향으로 운전됩니다. 종축이 속도 동기 운전중 상태에서도 속도 동기 비율을 변경할 수 있습니다.

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
| XSSS | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음의 경우는 에러가 발생하여, 속도 동기 운전이 실행되지 않습니다.
- M 코드 신호가 On된 축은, 에러(코드:353)가 발생하여 속도 동기 명령의 실행할 수 없습니다.
- 속도 동기 명령을 실행하기 전에, M 코드 해제(XMOF) 명령을 실행하여 M 코드를 Off로 변경하여 주십시오.
- 주축과 종축을 같게 설정하며, 에러(코드:355)가 발생합니다.
- 속도 동기 비율에 의하여 종축의 속도가 속도 제한치를 벗어나는 경우는 에러(코드:357)이 발생합니다.
- 종축이 속도 동기 운전중에 속도 제한치를 벗어나는 경우는 기본 파라미터의‘급정지시 감속시간’으로 감속 정지합니다.
- 속도 동기 명령으로 종축의 속도를 변경하는 경우에, 변경 전후의 속도 차이가 크면 장비와 기계에 충격이 발생할 수 있으므로 주의하시기 바랍니다.
- 종축에 속도 동기 명령을 실행하고 주축에 원점 복귀 명령을 실행할 수 있으나, 주축이 원점 복귀 동작중일 때는 종축에 속도 동기 명령을 실행할 수 없습니다.
- 단, LS ELECTRIC의 서보(L7N/L7NH/L7NHF/PEGA시리즈)를 사용하는 경우에는, 주축이 원점 복귀 동작중일 때도 종축에 속도 동기 명령을 실행할 수 있습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSSSEX source figure 1](assets/b77b555a2c2bf87f-NewItem3828.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44213xsss.htm` — §4.42.13; source SHA-256 `523455c05e20ef824fa320f7731b05ef8798bec0c2c48438f0bf1f100ce049cb`.
- Additional behavior/operand evidence: `44213xsss.htm`.

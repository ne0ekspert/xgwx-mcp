---
name: "xgk-cin"
description: "Use CIN with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4416cin.htm"]
---

# CIN

Manual section(s): 4.41.6. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `CIN`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `CIN`; the catalog records opcode `958` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 원호보간을 실행할 스텝번호 ※ XGB 위치:XGB 증설(0~150) | WORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 원호보간을 실행할 종축설정 (Bit0:X 축, Bit1:Y 축, Bit2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 원호보간 지령을 내리는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 n2에 설정된 축을 종축으로 하여 n1스텝을 원호보간 운전을 하는 명령입니다.
- n2에 원호보간을 실행할 종축을 설정할 때는 아래와 같이 각 Bit별로 할당된 축의 Bit를 셋(Set)하여 설정합니다.

| 15 ~ 3 | 2 | 1 | 0 |
| --- | --- | --- | --- |
| 미사용 | Z축(XGB는 미사용) | Y축 | X축 |

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
| CIN | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 다음과 같은 경우에 에러가 되어 중심점 지정 방식의 원호 보간 제어를 실행할 수 없습니다.
- (1) 주축 운전 데이터의 「종축설정」 값이 “축미설정”인 경우
- (2) 주축 운전 데이터의 「종축설정」 값을 주축 번호와 같게 설정한 경우
- (3) 주축 운전 데이터의 「종축설정」 값이 현재 네트워크에 연결되지 않은 축 번호인 경우
- 주축 또는 종축의 제어 단위를 “degree”로 설정한 경우
- 보조점으로 지정한 중심점이 시작 위치 또는 목표 위치와 같은 경우
- 계산된 원호의 반지름이 2147483647pls를 초과하는 경우

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![CIN source figure 1](assets/9dd09509129c1adb-NewItem3599.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4416cin.htm` — §4.41.6; source SHA-256 `7ee888b9012e6df2b9a2ba12c51d1465457166296d627668c5ae697f4b7f8de8`.
- Additional behavior/operand evidence: `4416cin.htm`.

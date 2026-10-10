---
name: "xgk-ist"
description: "Use IST with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4414ist.htm"]
---

# IST

Manual section(s): 4.41.4. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `IST`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `IST`; the catalog records opcode `954` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 내릴 축 (0:X 축, 1:Y 축, 2:Z 축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 기동할 스텝번호 (0~400) ※ XGB 위치:내장(0~80), XGB 증설(0~150) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PxxA를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 모듈에 저장된 운전 데이터의의 스텝 번호를 지정하여 위치 제어를 기동합니다.
- sl(슬롯번호)로 지정된 위치결정 모듈의 ax 축에 n1 스텝으로 운전합니다.
- 간접 기동 보조 데이터 설정

| 설정 항목 | 내용 |
| --- | --- |
| 운전 스텝 | 실행하고자 하는 운전 데이터의 스텝 번호를 설정합니다. (0 또는 1 ~ 400) |

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
| IST | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 감속 정지를 한 후 간접 기동 시 설정된 운전 스텝으로 위치결정 운전을 합니다.
- M 코드 모드 중 With모드를 사용하였을 경우 M 코드 On 신호를 “Off”해야만 다시 기동할 수 있습니다. M코드 해제(XMOF)」 명령으로 M 코드 On 신호를 “Off”한 후 기동을 실행하십시오.
- 내부 비상 정지, 외부 비상 정지 후 재 기동의 경우, 비상 정지에 의해 정지 된 되어 M코드 On 신호는 자동으로 “Off”되므로 간접 기동 시 설정된 운전 스텝으로 운전을 실행할 수 있습니다.
- 간접 기동의 운전 스텝에 0을 설정한 후 간접 기동 명령을 실행하면 현재 스텝 번호에 저장된 스텝의 운전 데이터를 실행합니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![IST source figure 1](assets/6f056bf9f5323a09-NewItem3592.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4414ist.htm` — §4.41.4; source SHA-256 `f17ce1de136d900dd5ca3df2fe485e64b73974816dc478a933d1d4724d7f6662`.
- Additional behavior/operand evidence: `4414ist.htm`.

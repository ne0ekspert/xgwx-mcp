---
name: "xgk-xlsetex"
description: "Use XLSETEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44261xlset.htm"]
---

# XLSETEX

Manual section(s): 4.42.61. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XLSETEX`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XLSETEX`; the catalog records opcode `1596` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 캠 지령을 내릴 종축 (1 ~ 8:1축 ~ 8축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 래치 허용/금지 (0:금지, 1:허용) | WORD | PMK, L, Z, N, D, R | Allowed |
| 4 | n2 | 래치 모드 (0:단일 트리거, 1:연속 트리거) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈의 외부 래치 기능을 허용/금지 하거나 래치 모드를 설정하기 위해 사용되는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축의 래치 기능을 허용/금지 하거나 래치 모드를 설정합니다.
- n1에 지정된 래치 허용/금지 항목에 따른 동작은 다음과 같습니다.
- 0:래치 금지
- 1:래치 허용 (“1” 이상의 값은 “1”과 동일하게 처리됩니다.)
- n2에 지정된 래치 모드 항목에 따른 동작은 다음과 같습니다.
- 0:단일 트리거 (래치 허용이 된 후 입력되는 첫번째 터치 프로브 1 신호에서만 현재 위치 래치)
- 1:연속 트리거 (래치 허용이 된 후 매 터치 프로브 1 신호마다 현재 위치 래치)
- (“1” 이상의 값은 “1”과 동일하게 처리됩니다.)
- 이 동작은 여러 스캔만에 완료될 수 있습니다.
- 래치 설정 명령은 네트워크 위치결정 모듈 XBF-PN04B/08B, XGF-PN4B/8B 전용 명령어 입니다.

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
| XLSET | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 단일 트리거 모드에서는 래치 기능이 허용된 후 첫번째 터치 프로브 1신호에서 래치 기능이 동작됩니다.
- 트리거 된 후에 입력되는 터치 프로브 1 신호에서는 래치 기능이 동작되지 않습니다.
- 래치 리셋 명령으로 단일 트리거 기능이 다시 허용된 후 입력되는 터치 프로브 1 신호에서 래치 기능이 다시 동작됩니다.
- 연속 트리거 모드에서는 래치 기능이 허용된 후 입력되는 모든 터치 프로브 1 신호에서 래치 기능이 동작됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XLSETEX source figure 1](assets/8ada31c5c77c70eb-NewItem3989.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44261xlset.htm` — §4.42.61; source SHA-256 `d76a8c8f771d252bf925cd364a2f97ea505ea39e60380e33b7db501421cdb596`.
- Additional behavior/operand evidence: `44261xlset.htm`.

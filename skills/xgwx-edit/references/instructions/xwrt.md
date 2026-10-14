---
name: "xgk-xwrt"
description: "Use XWRT with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44237xwrt.htm"]
---

# XWRT

Manual section(s): 4.42.37. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XWRT`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XWRT`; the catalog records opcode `1139` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 데이터를 저장할 축 (선택축) | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 선택축에 설정된 축들의 파라미터와 운전 데이터를 FRAM에 저장하는 명령입니다. 여기서 지령축은 형식적으로 존재하는 축입니다.
- 설정된 축의 현재 파라미터와 운전 데이터가 FRAM에 저장이 되며 전원이 Off되어도 유지됩니다.
- 파라미터/운전 데이터 저장 명령의 실행 횟수는 제한이 없습니다.
- 축 선택은 축에 해당하는 각 Bit를 Set하여 설정합니다.

| 15 ~ 8 Bit | 7Bit | 6Bit | 5Bit | 4Bit | 3Bit | 2Bit | 1Bit | 0Bit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 사용안함 | 8축 | 7축 | 6축 | 5축 | 4축 | 3축 | 2축 | 1축 |

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
| XWRT | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 해당 명령어는 1축당 실행 시간이 15ms가 소요 됩니다.
- 기타 다른 명령어는 축별로 아래와 같이 시간이 소요 됩니다.
- (1) XGF-PN8A :1.6ms (2축 사용 시) ~ 4.8ms (8축 사용 시)
- (2) XGF-PN4B :2ms (3축 사용 시) ~ 4ms (4축 사용 시)
- (3) XGF-PN8B :2ms (3축 사용 시) ~ 4ms (8축 사용 시)
- 가변데이터 쓰기 (XVWR) 명령으로 캠데이터를 수정한 경우에는 XWRT 실행 시 변경된 캠데이터가 FRAM이 아닌 FLASH에 저장됩니다.
- 쓰기 속도가 다소 오래 걸릴 수 있으니 참고하시기 바랍니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XWRT source figure 1](assets/62b545349b22f060-NewItem3737.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44237xwrt.htm` — §4.42.37; source SHA-256 `4d18f948a7f68b546a20ea1b0ebad8751923f633c0038b10ca802c37e386b25f`.
- Additional behavior/operand evidence: `44237xwrt.htm`.

---
name: "xgk-xsvsaveex"
description: "Use XSVSAVEEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["44257xsvsave.htm"]
---

# XSVSAVEEX

Manual section(s): 4.42.57. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XSVSAVEEX`.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `XSVSAVEEX`; the catalog records opcode `1719` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 서보 파라미터를 저장할 축 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축 수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 연결된 서보드라이브의 파라미터를 서보드라이브 내부의 EEPROM에 저장하는 명령입니다.
- ax에 설정한 명령을 내릴 축은 서보 파라미터를 저장하는 축과는 별개입니다.
- 따라서 서보 파라미터를 저장하려면 n1에 해당 축의 Bit를 Set해서 설정해야 합니다.
- n1에 설정된 축의 서보드라이브의 파라미터를 저장합니다.
- n1값의 각 Bit는 아래와 같이 각 축을 의미합니다. 축의 선택은 해당축의 Bit를 SET하여 설정합니다.

| 15 ~ 8 Bit | 7Bit | 6Bit | 5Bit | 4Bit | 3Bit | 2Bit | 1Bit | 0Bit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 사용안함 | 8축 | 7축 | 6축 | 5축 | 4축 | 3축 | 2축 | 1축 |

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
| XSVSAVE | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 해당 축이 운전 중인 경우에는 실행할 수 없습니다.
- 이 동작은 여러 스캔만에 완료될 수 있습니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XSVSAVEEX source figure 1](assets/5043f4a19f5e70a2-NewItem3970.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `44257xsvsave.htm` — §4.42.57; source SHA-256 `07f3c03b8825dcf477cdadbe5823c7482a1194d54096c4bacb018a2630847389`.
- Additional behavior/operand evidence: `44257xsvsave.htm`.

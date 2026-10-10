---
name: "xgk-xpttex"
description: "Use XPTTEX with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4429xptt.htm"]
---

# XPTTEX

Manual section(s): 4.42.9. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `XPTTEX`.

## Editing route

Use `insert_ladder_instruction` after calling `describe_operation` for its current schema.
The writer mnemonic is `XPTTEX`; the catalog records opcode `1693` and `3` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | ax | 지령을 실행할 축:1~8(실축) | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | n1 | 토크 지령 값 (-32768 ~ 32767) | INT | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 위치결정 모듈에 위치/토크 제어 전환 지령을 내리는 명령입니다.
- sl(위치결정 모듈의 슬롯번호)로 지정된 위치결정 모듈의 ax로 지정된 축에 n1으로 지정된 토크로 위치/토크 제어 전환 명령을 내립니다.
- 위치 제어 운전중인 상태에서 위치/토크 전환 제어가 실행되면 설정된 토크값(n1)의 토크 제어로 전환되고, 감속정지 등 정지 요인이 발생할 때까지 토크제어를 유지합니다.
- 토크 제어로 운전 중 「감속 정지」 또는 「스킵 운전」에 의해 정지 하거나 다음 운전을 계속할 수 있습니다
- 토크값의 범위는 -32768 ~ 32767까지 이며 단위는 [%]입니다. 토크 값의 허용 범위는 연결된 서보드라이브 종류에 따라 다릅니다. 일반적으로, 목표 토크 값은 최대 토크 설정으로 제한됩니다.
- (1) 토크 값이 양( + )일 때:정방향(현재 위치 증가 방향)으로 위치결정 운전
- (2) 토크 값이 음( - )일 때:역방향(현재 위치 감소 방향)으로 위치결정 운전

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
| XPTT | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 위치/토크 전환 제어 명령은 해당 축이 위치제어나 인칭 운전중일 경우에만 유효합니다.
- 설정된 목표 위치로 위치결정 되기까지 위치/토크 전환명령이 입력되지 않은 경우에는 감속정지 하여 위치결정 됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![XPTTEX source figure 1](assets/9673c779b781a580-NewItem3814.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4429xptt.htm` — §4.42.9; source SHA-256 `61fbdb280f0e44658e6dec14e4e512bf056c34bfe3fa913a4c7e41768d40ae3e`.
- Additional behavior/operand evidence: `4429xptt.htm`.

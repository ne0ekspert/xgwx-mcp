---
name: "xgk-getmp"
description: "Use GETMP with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
metadata:
  manual: "XGK(B)InstructionHelp_Kr_V3.5.chm"
  manual_sha256: "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"
  manual_pages: ["4431getmgetmp.htm"]
---

# GETMP

Manual section(s): 4.43.1. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `GETMP`.

Execution form: the source explicitly pairs this command as `(P)`; `GETMP` is the pulse-type P form. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.

## Editing route

Use `get_ir` → `apply_patch` for this instruction edit. `insert_ladder_instruction` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints.
The writer mnemonic is `GETMP`; the catalog records opcode `790` and `4` operands in order.
Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).

## Operands

| Order | Operand | Description (Korean manual) | Reviewed types | Device areas | Constant |
| --- | --- | --- | --- | --- | --- |
| 1 | sl | 위치 모듈이 장착된 베이스(x)와 슬롯(y) 번호:h00xy | WORD | Unknown | Unknown |
| 2 | S | 모션 모듈의 고정영역 선두 어드레스 | WORD | PMK, L, Z, N, D, R | Allowed |
| 3 | D | CPU내의 Device 이름 및 번호 | DWORD | PMK, L, Z, N, D, R | Allowed |
| 4 | N | 읽을 데이터의 개수 | WORD | PMK, L, Z, N, D, R | Allowed |

These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.

## CPU and acceptance

No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility.
Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.

## Manual behavior notes (Korean)

- ※ 오퍼랜드 설정 범위는 XGF-PN8B를 기준으로 합니다. 축에 대한 오퍼랜드(ax)의 설정 범위는 제품에서 지원하는 제어 축수에 따라 다르니, 제품 사용설명서를 참조하여 주십시오.
- 이 명령은 모션모듈의 데이터를 Read하고자 하는 경우 사용되는 명령어 입니다.
- sl(특수 모듈의 슬롯번호)로 지정된 모션 모듈의 메모리(S로 지정 :어드레스)로부터 N개 만큼의 Double Word 데이터를 D로 지정된 CPU영역으로 Read합니다.

## Manual flags

| 플래그 | 내 용 | 디바이스 번호 |
| --- | --- | --- |
| PUT/GET 에러 | 1. 지정한 베이스/슬롯에 모듈이 없거나 모션 모듈이 아닌 경우 2. PUT/GET 명령을 제대로 실행하지 못 했을 경우 3. CPU별 지원하는 max base 범위를 넘어선 경우 4. 지정한 슬롯에 장착된 모듈에 S로 지정한 어드레스가 없을 경우 5. ax값이 범위를 벗어날 경우 | F0015~F0022 |

## Manual applicability and restrictions

- 적용 기종 : XGK, XBC, XBM, XBF
- S로 지정한 어드레스부터 N개까지의 영역이 해당 블록을 넘어갈 경우 에러가 발생합니다.
- 이 에러는 현재 XG5000에서 입력할 수 없도록 차단되어 있습니다.
- 지정한 슬롯에 장착된 모듈이 모션 모듈이 아니거나, 장착된 모듈에 S로 지정한 어드레스가 없을 경우 PUT/GET 에러(F0015~F0022)와 연산에러(F110)를 함께 셋(Set)합니다. 이는 특수 모듈의 특성에 따라 고정영역 주소가 다를 수 있기 때문입니다.
- 지정한 슬롯에 장착된 모듈이 없는 경우 PUT/GET 에러(F0015~F0022)만 셋(Set)합니다.

## Manual flag and step matrix

The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.

| Manual instruction group | Native steps | 에러 (F110) | 제로 (F111) | 캐리 (F112) |
| --- | --- | --- | --- | --- |
| GETM(P) | 4~7 | O | - | - |

## Additional source notes after examples

The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.

- 서보파라미터 읽기 명령 실행 후 최소4ms 후에 서보파라미터R/W 처리 중 상태가 Off 일때GETM(또는GETMP)명령을 실행해야 합니다.
- 모듈에서 서보파라미터를 읽은 데이터가 데이터 공용 영역에 업데이트 된 다음 반영된 값을 읽어올 수 있습니다.
- 가변 데이터 읽기 명령 실행 후 최소 2ms 후에 GETM 명령을 실행해야 읽은 데이터가 데이터 공용영역에 저장됩니다.

## Referenced manual figures

These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.

![GETMP source figure 1](assets/a65d4525bfc60ee0-NewItem4034.png)

![GETMP source figure 2](assets/65cb127e509f9a5c-NewItem4033.png)

## Provenance

Source: `XGK(B)InstructionHelp_Kr_V3.5.chm`, SHA-256 `b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0`.

- `4431getmgetmp.htm` — §4.43.1; source SHA-256 `2334ba4a20072859d68f0926c8e8b62492911563a020aaa3e5de2d8e49e527d6`.
- Additional behavior/operand evidence: `4431getmgetmp.htm`, `245.htm`.

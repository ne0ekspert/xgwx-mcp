# XG5000 native acceptance, 2026-10-14

The revision-scoped IR writers were exercised in XG5000 4.82.1.0 (2026-07-20) in an offline Windows VM. Each generated project was opened, checked with all programs selected, saved under a new name, and parsed again on the host.

| Case | Covered edits | XG5000 Check Program | Decoded Save As comparison |
| --- | --- | --- | --- |
| `iec` | Project metadata, first IEC local, IEC contact/parallel/rung/blank lifecycle | 0 errors, 0 warnings | ProgramData, symbols, and all exported IR sections equal |
| `contact` | IEC parallel and positioned contact insertion | 0 errors, 0 warnings | ProgramData, symbols, and all exported IR sections equal |
| `repair` | IEC contact deletion and wire-gap repair | 0 errors, 0 warnings | ProgramData, symbols, and all exported IR sections equal |
| `io` | XGK modules, scoped I/O generation, ladder edit, and blank-row insertion | 0 errors, 0 warnings | ProgramData, symbols, and all exported IR sections equal |

The compared IR sections were `project`, `programs`, `modules`, `variables`, and the edited `program`. Revision header lines were omitted because the native Save As container has a different file hash. The decoded ProgramData and symbol payloads were byte-identical in every case.

The first run found that inserting the first IEC local into a native empty symbol table produced locally decodable base64 but omitted XG5000's `Compressed="1"` and typed `bin.base64` metadata. XG5000 rejected that project. The writer now materializes the same compressed table metadata as XG5000. A native-created first-local fixture verifies both the metadata and decoded record bytes.

The full validation after the correction was:

- `libxgwx`: 141 passed, 132 ignored, 0 failed.
- `xgwx-mcp`: 161 passed, 0 failed.
- XG5000: all four generated cases opened, checked, and saved successfully.
- Host comparison: decoded ProgramData, symbol payloads, and five IR exports were preserved for all four cases.

This matrix covers the paths listed above. It does not claim native execution on a PLC or universal coverage of every guarded IEC layout, SFC writer, ST/IL writer, or specialized native writer.

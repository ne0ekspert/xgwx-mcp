# Native blank project seeds

These files are copied verbatim from the sibling library's public, blank XG5000
fixtures. They contain no user application. `manifest.json` records each source
and SHA-256. The server embeds the bytes at compile time; no fixture checkout or
arbitrary client-selected template path is needed at runtime.

- `xgk_ld.xgwx`: `libxgwx/fixtures/empty-projects/new-xgk.xgwx`, initially XGK-CPUSN.
- `xgi_ld.xgwx`: `libxgwx/fixtures/empty-projects/new-xgi.xgwx`, initially XGI-CPUE.
- `xgi_sfc.xgwx`: `libxgwx/fixtures/sfc/new-xgi-sfc.xgwx`, initially XGI-CPUE.

The ladder seeds were created/saved in native XG5000; the SFC seed is the native
empty-chart fixture checked by the library's SFC tests. Creation preserves seed
project/configuration/program identities. Defaults are project `NewProject`, PLC
`LSPLC`, and program `NewProgram`; project/program names can be selected through
public writers at creation. It does not synthesize fresh
GUIDs or rewrite undocumented parameters/caches. Initial changes use public
library writers. CPU selection retains existing migration/hardware guards;
`list_templates` reports CPU/language creation choices accepted for each seed.

Blank programs/charts can be incomplete. Local verified serialization of a new
file does not establish XG5000 Open → Check Program → Save As acceptance or PLC
execution. No runtime template pruning or automatic replacement is performed.

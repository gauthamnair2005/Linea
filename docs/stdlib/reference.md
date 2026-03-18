# Standard Library Reference

- [math](math.md)
- [strings](strings.md)
- [io](io.md)
- [ml](ml.md)
- [datasets](datasets.md)
- `hash`: `sha256`, `sha512`, `md5`, `withSalt`, `secureEquals`
- `security`: random token/bytes, constant-time compare, password hash/verify, strength score
- `db`: SQLite helpers (`open`, `close`, `execute`, `query`, `initSecure`, `unlock`)
- `fileio`: explicit text/file/dir operations (`readText`, `writeText`, `appendText`, `listDir`, `sizeBytes`)
- `lowlevel`: bitwise operators and little-endian byte packing (`toBytesLE`, `fromBytesLE`)
- `git`: simple Git actions (`status`, `currentBranch`, `log`, `add`, `commit`, `push`, `pull`, `checkout`)
- `fun`: random/fun helpers (`coinFlip`, `rollDice`, `randomEmoji`, `randomJoke`, `choose`)
- `uuid`: identifier helpers (`v4`, `short`)
- `webserver`: HTTP serving (`serveText`, `serveJson`, `serveStatic`)
- `framework`: Django-like project and route tooling (`newProject`, `addRoute`, `runDevServer`)
- `blockchain`: chain utilities (`sha256`, `merkleRoot`, `mineBlock`, `validateLink`)
- `gpu_tools`: adapter/vendor/iGPU helpers (`adapters`, `bestAdapter`, `vendorName`, `hasIGPU`)
- `memory`: handle-based memory ops (`alloc`, `free`, `readU8`, `writeU8`, `copy`, `stats`)

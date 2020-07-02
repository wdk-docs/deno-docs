---
title: "相关检验 - info"
linkTitle: ""
weight: 76
type: "docs"
description: >
  `deno info [URL]` 将检查ES模块及其所有依赖.
---

```shell
deno info https://deno.land/std@0.52.0/http/file_server.ts
Download https://deno.land/std@0.52.0/http/file_server.ts
...
local: /Users/deno/Library/Caches/deno/deps/https/deno.land/5bd138988e9d20db1a436666628ffb3f7586934e0a2a9fe2a7b7bf4fb7f70b98
type: TypeScript
compiled: /Users/deno/Library/Caches/deno/gen/https/deno.land/std@0.52.0/http/file_server.ts.js
map: /Users/deno/Library/Caches/deno/gen/https/deno.land/std@0.52.0/http/file_server.ts.js.map
deps:
https://deno.land/std@0.52.0/http/file_server.ts
  ├─┬ https://deno.land/std@0.52.0/path/mod.ts
  │ ├─┬ https://deno.land/std@0.52.0/path/win32.ts
  │ │ ├── https://deno.land/std@0.52.0/path/_constants.ts
  │ │ ├─┬ https://deno.land/std@0.52.0/path/_util.ts
  │ │ │ └── https://deno.land/std@0.52.0/path/_constants.ts
  │ │ └─┬ https://deno.land/std@0.52.0/testing/asserts.ts
  │ │   ├── https://deno.land/std@0.52.0/fmt/colors.ts
  │ │   └── https://deno.land/std@0.52.0/testing/diff.ts
  │ ├─┬ https://deno.land/std@0.52.0/path/posix.ts
  │ │ ├── https://deno.land/std@0.52.0/path/_constants.ts
  │ │ └── https://deno.land/std@0.52.0/path/_util.ts
  │ ├─┬ https://deno.land/std@0.52.0/path/common.ts
  │ │ └── https://deno.land/std@0.52.0/path/separator.ts
  │ ├── https://deno.land/std@0.52.0/path/separator.ts
  │ ├── https://deno.land/std@0.52.0/path/interface.ts
  │ └─┬ https://deno.land/std@0.52.0/path/glob.ts
  │   ├── https://deno.land/std@0.52.0/path/separator.ts
  │   ├── https://deno.land/std@0.52.0/path/_globrex.ts
  │   ├── https://deno.land/std@0.52.0/path/mod.ts
  │   └── https://deno.land/std@0.52.0/testing/asserts.ts
  ├─┬ https://deno.land/std@0.52.0/http/server.ts
  │ ├── https://deno.land/std@0.52.0/encoding/utf8.ts
  │ ├─┬ https://deno.land/std@0.52.0/io/bufio.ts
  │ │ ├─┬ https://deno.land/std@0.52.0/io/util.ts
  │ │ │ ├── https://deno.land/std@0.52.0/path/mod.ts
  │ │ │ └── https://deno.land/std@0.52.0/encoding/utf8.ts
  │ │ └── https://deno.land/std@0.52.0/testing/asserts.ts
  │ ├── https://deno.land/std@0.52.0/testing/asserts.ts
  │ ├─┬ https://deno.land/std@0.52.0/async/mod.ts
  │ │ ├── https://deno.land/std@0.52.0/async/deferred.ts
  │ │ ├── https://deno.land/std@0.52.0/async/delay.ts
  │ │ └─┬ https://deno.land/std@0.52.0/async/mux_async_iterator.ts
  │ │   └── https://deno.land/std@0.52.0/async/deferred.ts
  │ └─┬ https://deno.land/std@0.52.0/http/_io.ts
  │   ├── https://deno.land/std@0.52.0/io/bufio.ts
  │   ├─┬ https://deno.land/std@0.52.0/textproto/mod.ts
  │   │ ├── https://deno.land/std@0.52.0/io/util.ts
  │   │ ├─┬ https://deno.land/std@0.52.0/bytes/mod.ts
  │   │ │ └── https://deno.land/std@0.52.0/io/util.ts
  │   │ └── https://deno.land/std@0.52.0/encoding/utf8.ts
  │   ├── https://deno.land/std@0.52.0/testing/asserts.ts
  │   ├── https://deno.land/std@0.52.0/encoding/utf8.ts
  │   ├── https://deno.land/std@0.52.0/http/server.ts
  │   └── https://deno.land/std@0.52.0/http/http_status.ts
  ├─┬ https://deno.land/std@0.52.0/flags/mod.ts
  │ └── https://deno.land/std@0.52.0/testing/asserts.ts
  └── https://deno.land/std@0.52.0/testing/asserts.ts
```

相关检验工作与任何本地或远程 ES 模块。

## 缓存租金

`deno info` 可用于显示有关的缓存位置信息:

```shell
deno info
DENO_DIR location: "/Users/deno/Library/Caches/deno"
Remote modules cache: "/Users/deno/Library/Caches/deno/deps"
TypeScript compiler cache: "/Users/deno/Library/Caches/deno/gen"
```

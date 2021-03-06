---
title: "标准库"
linkTitle: ""
weight: 5
type: "docs"
description: >
  Deno提供了一套由核心团队审核，并保证与Deno工作的标准模块。
---

标准库可在: https://deno.land/std/

## 版本控制和稳定性

Standard library is not yet stable and therefore it is versioned differently
than Deno. For latest release consult https://deno.land/std/ or
https://deno.land/std/version.ts. The standard library is released each time
Deno is released.

We strongly suggest to always use imports with pinned version of standard
library to avoid unintended changes. For example, rather than linking to the
master branch of code, which may change at any time, potentially causing
compilation errors or unexpected behavior:

```typescript
// imports from master, this should be avoided
import { copy } from "https://deno.land/std/fs/copy.ts";
```

instead, used a version of the std library which is immutable and will not
change:

```typescript
// imports from v0.50.0 of std, never changes
import { copy } from "https://deno.land/std@0.50.0/fs/copy.ts";
```

## 故障排除

一些标准库使用不稳定 Deno API 提供的模块。

Trying to run such modules without `--unstable` CLI flag ends up with a lot of
TypeScript errors suggesting that some APIs in the `Deno` namespace do not
exist:

```typescript
// main.ts
import { copy } from "https://deno.land/std@0.50.0/fs/copy.ts";

copy("log.txt", "log-old.txt");
```

```shell
$ deno run --allow-read --allow-write main.ts
Compile file:///dev/deno/main.ts
Download https://deno.land/std@0.50.0/fs/copy.ts
Download https://deno.land/std@0.50.0/fs/ensure_dir.ts
Download https://deno.land/std@0.50.0/fs/_util.ts
error: TS2339 [ERROR]: Property 'utime' does not exist on type 'typeof Deno'.
    await Deno.utime(dest, statInfo.atime, statInfo.mtime);
               ~~~~~
    at https://deno.land/std@0.50.0/fs/copy.ts:90:16

TS2339 [ERROR]: Property 'utimeSync' does not exist on type 'typeof Deno'.
    Deno.utimeSync(dest, statInfo.atime, statInfo.mtime);
         ~~~~~~~~~
    at https://deno.land/std@0.50.0/fs/copy.ts:101:10
```

Solution to that problem requires adding `--unstable` flag:

```shell
deno run --allow-read --allow-write --unstable main.ts
```

To make sure that API producing error is unstable check
[`lib.deno.unstable.d.ts`](https://github.com/denoland/deno/blob/master/cli/js/lib.deno.unstable.d.ts)
declaration.

This problem should be fixed in the near future. Feel free to omit the flag if
the particular modules you depend on compile successfully without it.

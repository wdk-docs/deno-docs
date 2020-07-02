---
title: "运行时"
linkTitle: ""
weight: 3
type: "docs"
description: >
  所有运行时函数文档（网页的 API +`Deno`全球）可以
  在[`doc.deno.land`](https://doc.deno.land/https/github.com/denoland/deno/releases/latest/download/lib.deno.d.ts)中找到.
---

## Web APIs

对于在网络标准已经存在，如 API 的`fetch` HTTP 请求，Deno 使用这些，而不是发明一
种新的专有 API。

对于实现的 Web API 的详细文档可以
在[doc.deno.land](https://doc.deno.land/https/raw.githubusercontent.com/denoland/deno/master/cli/js/lib.deno.shared_globals.d.ts)中
找到. 另外, a full list of the Web APIs which Deno implements is also available
[in the repository](https://github.com/denoland/deno/blob/master/cli/js/web/README.md).

The TypeScript definitions for the implemented web APIs can be found in the
[`lib.deno.shared_globals.d.ts`](https://github.com/denoland/deno/blob/master/cli/js/lib.deno.shared_globals.d.ts)
and
[`lib.deno.window.d.ts`](https://github.com/denoland/deno/blob/master/cli/js/lib.deno.window.d.ts)
files.

Definitions that are specific to workers can be found in the
[`lib.deno.worker.d.ts`](https://github.com/denoland/deno/blob/master/cli/js/lib.deno.worker.d.ts)
file.

## `Deno` global

All APIs that are not web standard are contained in the global `Deno` namespace.
It has the APIs for reading from files, opening TCP sockets, and executing
subprocesses, etc.

The TypeScript definitions for the Deno namespaces can be found in the
[`lib.deno.ns.d.ts`](https://github.com/denoland/deno/blob/master/cli/js/lib.deno.ns.d.ts)
file.

The documentation for all of the Deno specific APIs can be found on
[doc.deno.land](https://doc.deno.land/https/raw.githubusercontent.com/denoland/deno/master/cli/js/lib.deno.ns.d.ts).

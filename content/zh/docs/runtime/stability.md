---
title: "稳定性"
linkTitle: ""
weight: 31
type: "docs"
description: >
  作为Deno1.0.0中，`Deno`命名空间的API是稳定的。
  我们将努力使1.0.0下的代码工作这意味着继续在未来的版本的工作。
---

However, not all of Deno's features are ready for production yet. Features which
are not ready, because they are still in draft phase, are locked behind the
`--unstable` command line flag.

```shell
deno run --unstable mod_which_uses_unstable_stuff.ts
```

传递此标志做了几件事情:

- 它能够在运行时使用不稳定的 API。
- It adds the
  [`lib.deno.unstable.d.ts`](https://doc.deno.land/https/raw.githubusercontent.com/denoland/deno/master/cli/js/lib.deno.unstable.d.ts)
  file to the list of TypeScript definitions that are used for type checking.
  This includes the output of `deno types`.

You should be aware that many unstable APIs have **not undergone a security
review**, are likely to have **breaking API changes** in the future, and are
**not ready for production**.

## 标准模块

Deno's standard modules (https://deno.land/std/) are not yet stable. We
currently version the standard modules differently from the CLI to reflect this.
Note that unlike the `Deno` namespace, the use of the standard modules do not
require the `--unstable` flag (unless the standard module itself makes use of an
unstable Deno feature).

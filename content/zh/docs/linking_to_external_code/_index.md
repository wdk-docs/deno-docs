---
title: "链接到第三方代码"
linkTitle: "第三方代码"
weight: 4
type: "docs"
description: >
  在[开始](./getting_started.md)部分中，我们看到 Deno 可以从 URL 中执行脚本。
  像浏览器的JavaScript，Deno可以从网址直接导入库。 本例使用URL导入断言库:
---

**test.ts**

```ts
import { assertEquals } from "https://deno.land/std/testing/asserts.ts";

assertEquals("hello", "hello");
assertEquals("world", "world");

console.log("Asserted! ✓");
```

尝试运行此:

```shell
$ deno run test.ts
Compile file:///mnt/f9/Projects/github.com/denoland/deno/docs/test.ts
Download https://deno.land/std/testing/asserts.ts
Download https://deno.land/std/fmt/colors.ts
Download https://deno.land/std/testing/diff.ts
Asserted! ✓
```

请注意，我们没有提供该节目的`--allow-net`标志，但它访问网络。运行时必须下载的进
口特殊的访问，并将它们缓存到磁盘。

Deno caches remote imports in a special directory specified by the `DENO_DIR`
environment variable. It defaults to the system's cache directory if `DENO_DIR`
is not specified. The next time you run the program, no downloads will be made.
If the program hasn't changed, it won't be recompiled either. The default
directory is:

- On Linux/Redox: `$XDG_CACHE_HOME/deno` or `$HOME/.cache/deno`
- On Windows: `%LOCALAPPDATA%/deno` (`%LOCALAPPDATA%` = `FOLDERID_LocalAppData`)
- On macOS: `$HOME/Library/Caches/deno`
- If something fails, it falls back to `$HOME/.deno`

## FAQ

### 如何导入模块的特定版本？

Specify the version in the URL. For example, this URL fully specifies the code
being run: `https://unpkg.com/liltest@0.0.5/dist/liltest.js`.

### 这似乎笨拙对进口网址随处可见。

> What if one of the URLs links to a subtly different version of a library?

> Isn't it error prone to maintain URLs everywhere in a large project?

The solution is to import and re-export your external libraries in a central
`deps.ts` file (which serves the same purpose as Node's `package.json` file).
For example, let's say you were using the above assertion library across a large
project. Rather than importing `"https://deno.land/std/testing/asserts.ts"`
everywhere, you could create a `deps.ts` file that exports the third-party code:

**deps.ts**

```ts
export {
  assert,
  assertEquals,
  assertStrContains,
} from "https://deno.land/std/testing/asserts.ts";
```

And throughout the same project, you can import from the `deps.ts` and avoid
having many references to the same URL:

```ts
import { assertEquals, runTests, test } from "./deps.ts";
```

This design circumvents a plethora of complexity spawned by package management
software, centralized code repositories, and superfluous file formats.

### 我怎么能相信，可以更改 URL？

By using a lock file (with the `--lock` command line flag), you can ensure that
the code pulled from a URL is the same as it was during initial development. You
can learn more about this
[here](./linking_to_external_code/integrity_checking.md).

### 但是，如果该 URL 的主机出现什么了？源将不可用。

This, like the above, is a problem faced by _any_ remote dependency system.
Relying on external servers is convenient for development but brittle in
production. Production software should always vendor its dependencies. In Node
this is done by checking `node_modules` into source control. In Deno this is
done by pointing `$DENO_DIR` to some project-local directory at runtime, and
similarly checking that into source control:

```shell
# Download the dependencies.
DENO_DIR=./deno_dir deno cache src/deps.ts

# Make sure the variable is set for any command which invokes the cache.
DENO_DIR=./deno_dir deno test src

# Check the directory into source control.
git add -u deno_dir
git commit
```

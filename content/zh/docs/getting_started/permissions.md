---
title: "权限"
linkTitle: ""
weight: 25
type: "docs"
description: >
  Deno默认情况下的安全。
  因此，除非您特别启用它，一个Deno模块没有文件，网络或环境，例如访问。
  进入安全敏感区域或功能需要使用权限被授予命令行上Deno过程。
---

对于下面的例子, `mod.ts`已被授予只读文件系统访问. 它不能写，或执行任何其他安全敏
感的功能。

```shell
deno run --allow-read mod.ts
```

## 权限列表

下列权限可用:

- **-A, --allow-all** 允许所有权限。这将禁用所有的安全。
- **--allow-env** 允许环境访问为了这些事如获取和设置环境变量.
- **--allow-hrtime** Allow high resolution time measurement. High resolution
  time can be used in timing attacks and fingerprinting.
- **--allow-net=\<allow-net\>** Allow network access. You can specify an
  optional, comma separated list of domains to provide a allow-list of allowed
  domains.
- **--allow-plugin** Allow loading plugins. Please note that --allow-plugin is
  an unstable feature.
- **--allow-read=\<allow-read\>** Allow file system read access. You can specify
  an optional, comma separated list of directories or files to provide a
  allow-list of allowed file system access.
- **--allow-run** Allow running subprocesses. Be aware that subprocesses are not
  run in a sandbox and therefore do not have the same security restrictions as
  the deno process. Therefore, use with caution.
- **--allow-write=\<allow-write\>** Allow file system write access. You can
  specify an optional, comma separated list of directories or files to provide a
  allow-list of allowed file system access.

## 权限允许列表

Deno also allows you to control the granularity of some permissions with
allow-lists.

This example restricts file system access by allow-listing only the `/usr`
directory, however the execution fails as the process was attempting to access a
file in the `/etc` directory:

```shell
$ deno run --allow-read=/usr https://deno.land/std/examples/cat.ts /etc/passwd
error: Uncaught PermissionDenied: read access to "/etc/passwd", run again with the --allow-read flag
► $deno$/dispatch_json.ts:40:11
    at DenoError ($deno$/errors.ts:20:5)
    ...
```

Try it out again with the correct permissions by allow-listing `/etc` instead:

```shell
deno run --allow-read=/etc https://deno.land/std/examples/cat.ts /etc/passwd
```

`--allow-write` works the same as `--allow-read`.

## 网络访问:

_fetch.ts_:

```ts
const result = await fetch("https://deno.land/");
```

This is an example on how to allow-list hosts/urls:

```shell
deno run --allow-net=github.com,deno.land fetch.ts
```

If `fetch.ts` tries to establish network connections to any other domain, the
process will fail.

Allow net calls to any host/url:

```shell
deno run --allow-net fetch.ts
```

---
title: "脚本安装"
linkTitle: ""
weight: 72
type: "docs"
description: >
  杰诺提供`deno install`轻松地安装和分发可执行代码。
---

`deno install [OPTIONS...] [URL] [SCRIPT_ARGS...]` 将根据名字`EXE_NAME`安装可
在`URL`脚本。

This command creates a thin, executable shell script which invokes `deno` using
the specified CLI flags and main module. It is placed in the installation root's
`bin` directory.

Example:

```shell
$ deno install --allow-net --allow-read https://deno.land/std/http/file_server.ts
[1/1] Compiling https://deno.land/std/http/file_server.ts

✅ Successfully installed file_server.
/Users/deno/.deno/bin/file_server
```

要更改可执行文件的名称，用 `-n`/`--name`:

```shell
deno install --allow-net --allow-read -n serve https://deno.land/std/http/file_server.ts
```

可执行文件名默认情况下推断:

- 尝试把 URL 路径的文件干。 上面的例子将成为“file_server”。
- 如果该文件的茎是通用的东西像 'main', 'mod', 'index' or 'cli', 和路径没有父, 取
  父路 ​​ 径的文件名. 否则，与通用名定居。

要更改安装根目录, 用 `--root`:

```shell
deno install --allow-net --allow-read --root /usr/local https://deno.land/std/http/file_server.ts
```

安装根被确定，按优先级顺序:

- `--root` option
- `DENO_INSTALL_ROOT` environment variable
- `$HOME/.deno`

这些必须如果需要，可以手动添加到路径。

```shell
echo 'export PATH="$HOME/.deno/bin:$PATH"' >> ~/.bashrc
```

您必须指定将用于运行在安装时，将脚本权限。

```shell
deno install --allow-net --allow-read https://deno.land/std/http/file_server.ts -p 8080
```

上述命令创建一个可执行称为`file_server`与网络运行和读权限和结合到端口 8080。

对于好的做法，使用[`import.meta.main`](../examples/testing_if_main.md)成语中的可
执行脚本来指定入口点。

例:

```ts
// https://example.com/awesome/cli.ts
async function myAwesomeCli(): Promise<void> {
  -- snip --
}

if (import.meta.main) {
  myAwesomeCli();
}
```

当您创建一个可执行脚本，做到让用户知道通过添加例如安装命令到存储库:

```shell
# Install using deno install

$ deno install -n awesome_cli https://example.com/awesome/cli.ts
```

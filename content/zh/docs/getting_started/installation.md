---
title: "安装"
linkTitle: ""
weight: 21
type: "docs"
description: >
  Deno工作在MacOS，Linux和Windows.Deno是一个单一的二进制可执行文件.它没有外部依赖。
---

## 下载并安装

[deno_install](https://github.com/denoland/deno_install) 提供了方便的脚本下载并
安装二进制文件。

运用 Shell (macOS 和 Linux):

```shell
curl -fsSL https://deno.land/x/install/install.sh | sh
```

运用 PowerShell (Windows):

```shell
iwr https://deno.land/x/install/install.ps1 -useb | iex
```

运用 [Scoop](https://scoop.sh/) (Windows):

```shell
scoop install deno
```

运用 [Chocolatey](https://chocolatey.org/packages/deno) (Windows):

```shell
choco install deno
```

运用 [Homebrew](https://formulae.brew.sh/formula/deno) (macOS):

```shell
brew install deno
```

运用 [Cargo](https://crates.io/crates/deno) (Windows, macOS, Linux):

```shell
cargo install deno
```

Deno binaries can also be installed manually, by downloading a zip file at
[github.com/denoland/deno/releases](https://github.com/denoland/deno/releases).
These packages contain just a single executable file. You will have to set the
executable bit on macOS and Linux.

## 测试你的安装

To test your installation, run `deno --version`. If this prints the Deno version
to the console the installation was successful.

Use `deno help` to see help text documenting Deno's flags and usage. Get a
detailed guide on the CLI [here](./command_line_interface.md).

## 更新

To update a previously installed version of Deno, you can run:

```shell
deno upgrade
```

This will fetch the latest release from
[github.com/denoland/deno/releases](https://github.com/denoland/deno/releases),
unzip it, and replace your current executable with it.

You can also use this utility to install a specific version of Deno:

```shell
deno upgrade --version 1.0.1
```

## 从源代码编译

有关如何从源代码编译的信息可以在`Contributing`章。

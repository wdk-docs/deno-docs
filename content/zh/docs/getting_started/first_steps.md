---
title: "第一步"
linkTitle: ""
weight: 23
type: "docs"
description: >
  此页面包含一些例子来教你 Deno 的基本面。
---

本文假设你有一些 JavaScript 的先验知识，特别是关于 `async`/`await`。如果你的
JavaScript 的先验知识， 你可能要遵循
的[JavaScript 的基础知识](https://developer.mozilla.org/en-US/docs/Learn/JavaScript)指
南试图启动 Deno 之前。

## Hello World

Deno is a runtime for JavaScript/TypeScript which tries to be web compatible and
use modern features wherever possible.

Browser compatibility means a `Hello World` program in Deno is the same as the
one you can run in the browser:

```ts
console.log("Welcome to Deno 🦕");
```

Try the program:

```shell
deno run https://deno.land/std/examples/welcome.ts
```

## 使一个 HTTP 请求

Many programs use HTTP requests to fetch data from a webserver. Let's write a
small program that fetches a file and prints its contents out to the terminal.

Just like in the browser you can use the web standard
[`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) API to
make HTTP calls:

```ts
const url = Deno.args[0];
const res = await fetch(url);

const body = new Uint8Array(await res.arrayBuffer());
await Deno.stdout.write(body);
```

Let's walk through what this application does:

1. We get the first argument passed to the application, and store it in the
   `url` constant.
2. We make a request to the url specified, await the response, and store it in
   the `res` constant.
3. We parse the response body as an
   [`ArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer),
   await the response, and convert it into a
   [`Uint8Array`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)
   to store in the `body` constant.
4. We write the contents of the `body` constant to `stdout`.

Try it out:

```shell
deno run https://deno.land/std/examples/curl.ts https://example.com
```

You will see this program returns an error regarding network access, so what did
we do wrong? You might remember from the introduction that Deno is a runtime
which is secure by default. This means you need to explicitly give programs the
permission to do certain 'privileged' actions, such as access the network.

Try it out again with the correct permission flag:

```shell
deno run --allow-net=example.com https://deno.land/std/examples/curl.ts https://example.com
```

## 读取文件

Deno also provides APIs which do not come from the web. These are all contained
in the `Deno` global. You can find documentation for these APIs on
[doc.deno.land](https://doc.deno.land/https/github.com/denoland/deno/releases/latest/download/lib.deno.d.ts).

Filesystem APIs for example do not have a web standard form, so Deno provides
its own API.

In this program each command-line argument is assumed to be a filename, the file
is opened, and printed to stdout.

```ts
const filenames = Deno.args;
for (const filename of filenames) {
  const file = await Deno.open(filename);
  await Deno.copy(file, Deno.stdout);
  file.close();
}
```

The `copy()` function here actually makes no more than the necessary
kernel→userspace→kernel copies. That is, the same memory from which data is read
from the file, is written to stdout. This illustrates a general design goal for
I/O streams in Deno.

Try the program:

```shell
deno run --allow-read https://deno.land/std/examples/cat.ts /etc/passwd
```

## TCP 服务器

This is an example of a server which accepts connections on port 8080, and
returns to the client anything it sends.

```ts
const hostname = "0.0.0.0";
const port = 8080;
const listener = Deno.listen({ hostname, port });
console.log(`Listening on ${hostname}:${port}`);
for await (const conn of listener) {
  Deno.copy(conn, conn);
}
```

For security reasons, Deno does not allow programs to access the network without
explicit permission. To allow accessing the network, use a command-line flag:

```shell
deno run --allow-net https://deno.land/std/examples/echo_server.ts
```

To test it, try sending data to it with netcat:

```shell
$ nc localhost 8080
hello world
hello world
```

Like the `cat.ts` example, the `copy()` function here also does not make
unnecessary memory copies. It receives a packet from the kernel and sends it
back, without further complexity.

## 更多示例

你可以找到更多的例子，比如一个 HTTP 文件服务器，在`Examples`章。

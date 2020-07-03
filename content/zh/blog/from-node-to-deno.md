---
date: 2020-05-17
title: "从 Node 到 Deno"
linkTitle: "从 Node 到 Deno"
description: >
  我收集了一些 Node 中最常用的主题，并寻找 Deno 的替代方案。
author: Aral Roca(翻译：疯狂的技术宅)
---

> 原文: https://aralroca.com/blog/from-node-to-deno

> 译文: https://segmentfault.com/a/1190000022735796

首先我想说明，许多当前的 Node.js 模块都可以都可以用在 Deno 中。由于许多模块都是
可重用的，所以没有必要为所有方法去寻找替代方案。你可以访问 pika.dev 查找可以在
Deno 中使用的模块。

本文将涵盖以下内容：

## Electron

通过 Node.js，我们可以使用 Electron 创建桌面程序。 Electron 使用 Chromium 作为接
口来运行 Web 环境。但是 Electron 可以在 Deno 中使用吗？有其他选择吗？

Electron logo

好吧，现在 Electron 还不能在 Deno 下执行，必须寻找替代方案。由于 Deno 是用 Rust
写的，所以可以用 web-view rust 绑定 在 Deno 中运行桌面程序。

这样，我们可以使用本机操作系统的 webview 视图来运行任意 webview。

Repo: https://github.com/eliassjogreen/deno_webview

```ts
import { WebView } from "https://deno.land/x/webview/mod.ts";

const sharedOptions = { width: 400, height: 200, resizable: true, debug: true,
frameless: false, };

const webview1 = new WebView({ title: "Multiple deno_webview example", url:
`data:text/html, <html> <body> <h1>1</h1> </body> </html>`, ...sharedOptions,
});

const webview2 = new WebView({ title: "Multiple deno_webview example", url:
`data:text/html, <html> <body> <h1>2</h1> </body> </html>`, ...sharedOptions,
});

await Promise.all([webview1.run(), webview2.run()]); Deno desktop app
```

## Forever / PM2

Forever 和 PM2 是 CLI 工具，可以使给定脚本作为守护程序连续运行。与 Forever 不同
，PM2 更完整，还可以用作负载均衡器。两者在 Node.js 中都非常有用，但是我们可以在
Deno 中使用吗？

Forever 仅能用于 Node，不过我们可以借助 PM2 运行非 Node.js 脚本，所以可以将其用
于 Deno。

PM2 logo

创建一个 app.sh 文件

```sh
#!/bin/bash deno run -A myCode.ts
```

然后

```sh
pm2 start ./app.sh 用 PM2 运行 Deno
```

## Express / Koa

Express 和 Koa 是最知名的 Node 框架。他们以其强大的路由系统和 HTTP 辅助器（重定
向、缓存等）而闻名。可以在 Deno 中使用它们吗？答案是否...但是有一些替代方法。

Express and Koa logo

### Http（标准库）

Deno 自己的标准库已经能够满足 Express 或 Koa 提供的许多功能。
https://deno.land/std/http/

```ts
import { ServerRequest } from "https://deno.land/std/http/server.ts";
import { getCookies } from "https://deno.land/std/http/cookie.ts";

let request = new ServerRequest();
request.headers = new Headers();
request.headers.set("Cookie", "full=of; tasty=chocolate");

const cookies = getCookies(request);
console.log("cookies:", cookies);
```

但是声明路由的方法并没有什么吸引力，所以让我们看看更多的替代方案。

### Oak (第三方库)

受 Koa 启发，这是目前最优雅的解决方案之一。 https://github.com/oakserver/oak

```ts
import { Application } from "https://deno.land/x/oak/mod.ts";

const app = new Application();

app.use((ctx) => {
  ctx.response.body = "Hello World!";
});

await app.listen({ port: 8000 });
```

### Abc（第三方库）

类似于 Oak https://deno.land/x/abc。

```sh
import { Application } from "https://deno.land/x/abc/mod.ts";

const app = new Application();

app.static("/static", "assets");

app.get("/hello", (c) => "Hello!") .start({ port: 8080 });

```

### Deno-express（第三方库）

也许是和 Express Framework 最相似的替代方案。 https://github.com/NMathar/de...。

```ts
import * as exp from "https://raw.githubusercontent.com/NMathar/deno-express/master/mod.ts";
const port = 3000; const app = new exp.App();
app.use(exp.static\_("./public")); app.use(exp.bodyParser.json());
app.get("/api/todos", async (req, res) => { await res.json([{ name: "Buy some milk" }]); });
const server = await app.listen(port);
console.log(`app listening on port ${server.port}`);
```

## 数据库

### MongoDB

MongoDB 是有着巨大的可扩展性和灵活性的文档型数据库。在 JavaScript 生态中已被广泛
使用，使用它的许多技术栈（如 MEAN 或 MERN）都非常受欢迎。

MongoDB logo

所以可以将 MongoDB 与 Deno 结合使用。可以使用这个驱动程序
：https://github.com/manyuanron...。

```ts
import { init, MongoClient } from "https://deno.land/x/mongo@v0.6.0/mod.ts";
// Initialize the plugin await init();
const client = new MongoClient();
client.connectWithUri("mongodb://localhost:27017");
const db = client.database("test");
const users = db.collection("users");
// insert const insertId = await users.insertOne({ username: "user1", password:"pass1" });
// findOne const user1 = await users.findOne({ \_id: insertId });
// find const users = await users.find({ username: { \$ne: null } });
// aggregation const docs = await users.aggregation([ { $match: { username:"many" } }, { $group: { _id: "$username", total: { $sum: 1 } } } ]);

// updateOne const { matchedCount, modifiedCount, upsertedId } = await
users.updateOne( username: { $ne: null }, { $set: { username: "USERNAME" } } );
// deleteOne const deleteCount = await users.deleteOne({ \_id: insertId });
```

### PostgresSQL

PostgresSQL logo

像 MongoDB 一样，也有 PostgresSQL 的驱动：https://github.com/buildondat...。

```ts
import { Client } from "https://deno.land/x/postgres/mod.ts";

const client = new Client({
  user: "user",
  database: "test",
  hostname: "localhost",
  port: 5432,
});
await client.connect();
const result = await;
client.query("SELECT * FROM people;");
console.log(result.rows);
await;
client.end();
```

### MySQL / MariaDB

MySQL 和 MariaDB logo

与 MongoDB 和 PostgresSQL 一样，还有 MySQL/MariaDB 的驱动程序。

https://github.com/manyuanron...

```ts
import { Client } from "https://deno.land/x/mysql/mod.ts";

const client = await new Client().connect({ hostname: "127.0.0.1", username:"root", db: "dbname", poolSize: 3, // connection limit password: "password", });

let result = await client.execute(`INSERT INTO users(name) values(?)`, ["aralroca", ]); console.log(result); // { affectedRows: 1, lastInsertId: 1 }
```

### Redis

Redis logo

Redis 是最著名的缓存数据库，也有 Deno 驱动程序。

```ts
//github.com/keroxp/den... import { connect } from
https: "https://denopkg.com/keroxp/deno-redis/mod.ts";

const redis = await connect({ hostname: "127.0.0.1", port: 6379 });
const ok = await redis.set("example", "this is an example");
const example = await;
redis.get("example");
```

## Nodemon

Nodemon logo

Nodemon 用于在开发环境中用于监视文件中的更改，并自动重新启动服务器。这使 Node 开
发更加有趣，而无需手动重启服务器来查看应用的更改。它可以用在 Deno 中吗？

抱歉，不可以...但是有另外一种选择：Denon。https://github.com/eliassjogr...

可以像使用 deno run 一样用 Denon 来执行脚本。

```sh
denon server.ts
```

## Jest, Jasmine, Ava...

![Jasmine, Jest, Ava, Mocha logos](https://segmentfault.com/img/remote/1460000022735811)

在 Node.js 生态中，有许多测试用的工具。但是官方并没有提供测试 Node.js 代码的方法
。

在 Deno 中，有一种官方的方法，可以用测试标准库。

https://deno.land/std/testing

```ts
import { assertStrictEq } from "https://deno.land/std/testing/asserts.ts";

Deno.test("My first test", async () => {
  assertStrictEq(true, false);
});
```

这样运行测试：

```sh
deno test
```

## Webpack, Parcel, Rollup...

Webpack, Parcel, Rollup logos

Deno 的一个优势是无需打包器（例如 Webpack、 Parcel 或 Rollup）就可以使 ESmodules
与 TypeScript 在一起工作。

但是如果给定一个文件树，我们是否可以打成一个包，把所有内容放到一个文件中并在网络
上运行它呢？

当然可以。可以用 Deno 的 CLI 做到这一点，不需要第三方打包程序。

```sh
deno bundle myLib.ts myLib.bundle.js
```

然后就可以加载到浏览器中了：

```html
<script type="module">
  import * as myLib from "myLib.bundle.js";
</script>
```

## Prettier

Prettier logo

在过去的几年中，Prettier 在 JavaScript 生态系统中已广为人知，正是因为有它，你不
必再去担心格式化文件的麻烦。

实际上它仍然可以在 Deno 上使用，但是这失去了意义，因为 Deno 有自己的格式化程序。

可以用以下命令格式化文件：

```sh
deno fmt
```

## NPM Scripts

Npm scripts logo

在 Deno 中，package.json 不再存在。我很怀念在 package.json 中声明的脚本。

一个简单的解决方案是用 makefile ，并使用 make 命令执行。但是，如果你想念 npm 语
法，那么 Deno 有一个 npm 风格的脚本运行器：

https://github.com/umbopepato...

你可以用脚本去定义文件：

```yaml
# scripts.yaml
scripts:
  start: deno run --allow-net server.ts
  test: deno test --allow-net server_test.ts
```

执行：

```sh
vr run <SCRIPT>
```

另一种选择是 denox，与 Velociraptor 非常相似。

## Nvm

Version semantics

Nvm 是一个用于管理多个活动 Node 版本的 CLI，可以根据你的项目轻松升级或降级版本。

在 Deno 中 nvm 的替代物是 dvm。

https://github.com/axetroy/dvm

```sh
dvm use 1.0.0
```

## Npx

Npx 近年来非常流行，可以直接调用 npm 包内的模块。由于 Deno 是一个独立的生态，所
以不存在 npm 中的那些重多项目。那么我们不用 deno install
https://url-of-module.ts 而用 Deno 执行来执行它们呢？

以与运行项目相同的方式，而不是文件，放置模块的 URL：

```sh
deno run https://deno.land/std/examples/welcome.ts
```

如你所见，我们不仅需要记住模块的名称，还要记住整个 URL，这样用起来很困难。但是另
一方面，它提供了更多的灵活性，因为我们可以运行任何文件，而不仅仅是像 npx 这样在
package.json 中指定的文件。

## 在 Docker 中运行

Docker logo

要在 Docker 中运行 Deno，可以这样创建 Dockerfile：

```dockerfile
FROM hayd/alpine-deno:1.0.0

EXPOSE 1993 # Port.
WORKDIR /app
USER deno

COPY deps.ts .
RUN deno cache deps.ts # Cache the deps

ADD . .
RUN deno cache main.ts # main entrypoint.

CMD ["--allow-net", "main.ts"]
```

这样构建并运行：

```sh
docker build -t app . && docker run -it --init -p 1993:1993 app
```

Repo: https://github.com/hayd/deno-...

## 用于亚马逊 lambda 运算

Lambda symbol

要将 Deno 用于 AWS lambda，可以用 Deno STD 库中的模块
。https://deno.land/x/lambda。

```ts
import {
  APIGatewayProxyEvent,
  APIGatewayProxyResult,
  Context,
} from "https://deno.land/x/lambda/mod.ts";
export async function handler(
  event: APIGatewayProxyEvent,
  context: Context
): Promise<APIGatewayProxyResult> {
  return {
    body: `Welcome to deno ${Deno.version.deno} 🦕`,
    headers: { "content-type": "text/html;charset=utf8" },
    statusCode: 200,
  };
}
```

有趣的参考：

- 把 Deno 用在 Vercel 中：https://github.com/lucacasona...
- AWS 中的 Deno：https://blog.begin.com/deno-r...

## 结束语

我确定肯定会遗漏了一些 Node 主题以及它们对应的 Deno 替代方案，如果你有补充请在下
面留言。

探索所有可以用在 Deno 中的库：

- https://deno.land/std
- https://deno.land/x
- https://www.pika.dev/

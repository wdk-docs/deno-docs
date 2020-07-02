---
title: "输入映射"
linkTitle: ""
weight: 44
type: "docs"
description: >
  Deno 支持[输入映射](https://github.com/WICG/import-maps).
---

> 这是一种不稳定的特点。详细了解[不稳定的特点](../runtime/stability.md).

您可以使用带 `--importmap=<FILE>` CLI 标志导入映射。

当前限制:

- 单个导入地图
- 没有后备网址
- Deno 不支持`STD:`命名空间
- 仅支持`file:`, `http:` 和 `https:`方案

例:

**import_map.json**

```js
{
   "imports": {
      "fmt/": "https://deno.land/std@0.55.0/fmt/"
   }
}
```

**color.ts**

```ts
import { red } from "fmt/colors.ts";

console.log(red("hello world"));
```

然后:

```shell
$ deno run --importmap=import_map.json --unstable color.ts
```

要使用绝对入口盯着目录:

```json
// import_map.json

{
  "imports": {
    "/": "./"
  }
}
```

```ts
// main.ts

import { MyUtil } from "/util.ts";
```

您可以映射不同的目录: (例如 src)

```json
// import_map.json

{
  "imports": {
    "/": "./src"
  }
}
```

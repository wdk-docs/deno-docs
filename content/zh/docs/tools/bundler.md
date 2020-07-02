---
title: "捆-bundle"
linkTitle: ""
weight: 74
type: "docs"
description: >
  `deno bundle [URL]` 将输出一个JavaScript文件, 其中包括指定的输入的所有依赖.
---

例如:

```
> deno bundle https://deno.land/std/examples/colors.ts colors.bundle.js
Bundling "colors.bundle.js"
Emitting bundle to "colors.bundle.js"
9.2 kB emitted.
```

如果省略了文件，该束将被发送到`stdout`。

束可以只运行在杰诺任何其他模块:

```
deno run colors.bundle.js
```

输出是包含 ES 模块自，其中在命令行上提供的主模块的任何出口将可用。 例如，如果主
模块看起来是这样的:

```ts
export { foo } from "./foo.js";

export const bar = "bar";
```

It could be imported like this:

```ts
import { foo, bar } from "./lib.bundle.js";
```

束也可以在 Web 浏览器加载. 束是一个自包含的 ES 模块等`type`的属性必须被设置
为`"module"`. For example:

```html
<script type="module" src="website.bundle.js"></script>
```

或者你可以将它导入到另一个 ES 模块消耗的:

```html
<script type="module">
  import * as website from "website.bundle.js";
</script>
```

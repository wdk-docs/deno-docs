---
title: "文档生成"
linkTitle: ""
weight: 75
type: "docs"
description: >
  `deno doc` 随后的一个或多个源文件的列表将打印JSDoc文档每个模块的**出口**成员。
  Currently, only exports in the style `export <declaration>` and `export ...
  from ...` are are supported.
---

For example, given a file `add.ts` with the contents:

```ts
/**
 * Adds x and y.
 * @param {number} x
 * @param {number} y
 * @returns {number} Sum of x and y
 */
export function add(x: number, y: number): number {
  return x + y;
}
```

Running the Deno `doc` command, prints the function's JSDoc comment to `stdout`:

```shell
deno doc add.ts
function add(x: number, y: number): number
  Adds x and y. @param {number} x @param {number} y @returns {number} Sum of x and y
```

Use the `--json` flag to output the documentation in JSON format. This JSON
format is consumed by the
[deno doc website](https://github.com/denoland/doc_website) and used to generate
module documentation.

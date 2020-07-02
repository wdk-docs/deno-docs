---
title: "测试当前文件是否主程序"
linkTitle: "是否主程序"
weight: 108
type: "docs"
description: >
  To test if the current script has been executed as the main input to the
  program check `import.meta.main`.
---

```ts
if (import.meta.main) {
  console.log("main");
}
```

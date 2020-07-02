---
title: "运用 TypeScript"
linkTitle: ""
weight: 26
type: "docs"
description: >
  Deno supports both JavaScript and TypeScript as first class languages at
  runtime.
---

<!-- TODO(lucacasonato): text on 'just import .ts' -->

This means it requires fully qualified module names, including the extension (or
a server providing the correct media type). In addition, Deno has no "magical"
module resolution. Instead, imported modules are specified as files (including
extensions) or fully qualified URL imports. Typescript modules can be directly
imported. E.g.

```
import { Response } from "https://deno.land/std@0.53.0/http/server.ts";
import { queue } from "./collections.ts";
```

## 使用外部`type`定义

The out of the box TypeScript compiler though relies on both extension-less
modules and the Node.js module resolution logic to apply types to JavaScript
modules.

In order to bridge this gap, Deno supports three ways of referencing type
definition files without having to resort to "magic" resolution.

### 编译器提示

If you are importing a JavaScript module, and you know where the type definition
for that module is located, you can specify the type definition at import. This
takes the form of a compiler hint. Compiler hints inform Deno the location of
`.d.ts` files and the JavaScript code that is imported that they relate to. The
hint is `@deno-types` and when specified the value will be used in the compiler
instead of the JavaScript module. For example, if you had `foo.js`, but you know
that alongside of it was `foo.d.ts` which was the types for the file, the code
would look like this:

```ts
// @deno-types="./foo.d.ts"
import * as foo from "./foo.js";
```

The value follows the same resolution logic as importing a module, meaning the
file needs to have an extension and is relative to the current module. Remote
specifiers are also allowed.

The hint affects the next `import` statement (or `export ... from` statement)
where the value of the `@deno-types` will be substituted at compile time instead
of the specified module. Like in the above example, the Deno compiler will load
`./foo.d.ts` instead of `./foo.js`. Deno will still load `./foo.js` when it runs
the program.

### 在 JavaScript 文件三斜杠参考指令

如果您承载要通过`Deno`消耗模块, 并且要告知`Deno`有关`type`定义的位置, 您可以利用
在实际的代码中的三斜杠指令. 例如, 如果你有一个 JavaScript 模块，你想提供 Deno 与
发生在`type`定义的位置是旁边那个文件, 命名你的 JavaScript 模块`foo.js`可能是这样
的 :

```js
/// <reference types="./foo.d.ts" />
export const foo = "foo";
```

Deno 会看到这一点，编译器将使用时的类型检查文件`foo.d.ts`，虽然`foo.js`会在运行
时加载。该指令的值的分辨率遵循相同的分辨率逻辑导入模块，表示文件需要以具有延伸并
且相对于当前文件。远程符也是允许的。

### X-TypeScript-Types custom header

If you are hosting modules which you want to be consumed by Deno, and you want
to inform Deno the location of the type definitions, you can use a custom HTTP
header of `X-TypeScript-Types` to inform Deno of the location of that file.

The header works in the same way as the triple-slash reference mentioned above,
it just means that the content of the JavaScript file itself does not need to be
modified, and the location of the type definitions can be determined by the
server itself.

**并非支持所有的`type`定义。**

Deno will use the compiler hint to load the indicated `.d.ts` files, but some
`.d.ts` files contain unsupported features. Specifically, some `.d.ts` files
expect to be able to load or reference type definitions from other packages
using the module resolution logic. For example a type reference directive to
include `node`, expecting to resolve to some path like
`./node_modules/@types/node/index.d.ts`. Since this depends on non-relative
"magical" resolution, Deno cannot resolve this.

**Why not use the triple-slash type reference in TypeScript files?**

The TypeScript compiler supports triple-slash directives, including a type
reference directive. If Deno used this, it would interfere with the behavior of
the TypeScript compiler. Deno only looks for the directive in JavaScript (and
JSX) files.

## 自`TypeScript`编译器选项

In the Deno ecosystem, all strict flags are enabled in order to comply with
TypeScript's ideal of being `strict` by default. However, in order to provide a
way to support customization a configuration file such as `tsconfig.json` might
be provided to Deno on program execution.

You need to explicitly tell Deno where to look for this configuration by setting
the `-c` (or `--config`) argument when executing your application.

```shell
deno run -c tsconfig.json mod.ts
```

以下是目前允许设置及其默认值`Deno`:

```json
{
  "compilerOptions": {
    "allowJs": false,
    "allowUmdGlobalAccess": false,
    "allowUnreachableCode": false,
    "allowUnusedLabels": false,
    "alwaysStrict": true,
    "assumeChangesOnlyAffectDirectDependencies": false,
    "checkJs": false,
    "disableSizeLimit": false,
    "generateCpuProfile": "profile.cpuprofile",
    "jsx": "react",
    "jsxFactory": "React.createElement",
    "lib": [],
    "noFallthroughCasesInSwitch": false,
    "noImplicitAny": true,
    "noImplicitReturns": true,
    "noImplicitThis": true,
    "noImplicitUseStrict": false,
    "noStrictGenericChecks": false,
    "noUnusedLocals": false,
    "noUnusedParameters": false,
    "preserveConstEnums": false,
    "removeComments": false,
    "resolveJsonModule": true,
    "strict": true,
    "strictBindCallApply": true,
    "strictFunctionTypes": true,
    "strictNullChecks": true,
    "strictPropertyInitialization": true,
    "suppressExcessPropertyErrors": false,
    "suppressImplicitAnyIndexErrors": false,
    "useDefineForClassFields": false
  }
}
```

For documentation on allowed values and use cases please visit the
[typescript docs](https://www.typescriptlang.org/docs/handbook/compiler-options.html).

**Note**: 任何选项上面未列出的可能是没有通过 Deno 支持或不列为
deprecated/experimental 的 TypeScript 文档。

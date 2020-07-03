---
date: 2020-06-19
title: "什么是最好的 Deno Web框架？"
linkTitle: "Deno 框架"
description: >
  One of the common use-cases for any language is it's HTTP server capabilities.
author: Craig Morten
---

一个常见的用例的任何语言是它的 HTTP 服务器功能。 一般来说，我们往往看到社会上任
何一个给定的语言会聚或几个 web 框架，其提供的功能 VS 发展成熟和后盾的性能和显示
的迹象最好的权衡。

Because Deno is so new it can be difficult to know which of the arising web
server frameworks is the one to use! On just the Deno Land third party modules
page there are 18 hits for web framework and 33 hits for server.

In this article I have tried to review the majority of frameworks, across
several key factors, to help you can make an informed decision about which is
the best for you!

I encourage to also read to the end where I touch on some frameworks I didn't
fully review as well as any suggestions made in the comments!

## 社区

围绕一个框架，一个健康的社区确实有助于使用框架更容易做出。因为它是这么早于它不是
特别容易确定哪个框架有最好的社会，但我们可以推断出可能从 GitHub 使用的一些感觉。

下面是一些 GitHub 的统计信息:

| 框架         | 星数 | 叉数 | 观数 | 未决问题 | 关闭问题 |
| ------------ | ---- | ---- | ---- | -------- | -------- |
| Abc          | 367  | 43   | 18   | 2        | 33       |
| Ako          | 5    | 0    | 2    | 3        | 0        |
| Aqua         | 32   | 1    | 1    | 2        | 12       |
| Attain       | 21   | 1    | 1    | 6        | 16       |
| Denotrain    | 54   | 9    | 3    | 1        | 9        |
| Drash        | 425  | 13   | 17   | 6        | 76       |
| Fastro       | 6    | 0    | 2    | 0        | 2        |
| Fen          | 69   | 2    | 9    | 0        | 1        |
| http_wrapper | 1    | 1    | 1    | 2        | 1        |
| Oak          | 1715 | 111  | 32   | 7        | 101      |
| Opine        | 51   | 2    | 7    | 0        | 2        |
| Pogo         | 238  | 21   | 15   | 7        | 11       |
| Servest      | 492  | 29   | 12   | 4        | 8        |
| Snowlight    | 10   | 0    | 3    | 0        | 0        |

- Stars - These tend to give a reasonable impression of what other people in the
  community are using so can be a reasonable indicator of what is good. Take
  care though, early on in a new language what was there first tends to start
  off with the most stars, but isn't necessarily what is the best now!
- Forks - A high number of forks tend to indicate that people are actively using
  and/or contributing to a framework.
- Watches - A high number of people watching a repository means that there is a
  community actively interested in receiving notifications for it's development.
- Open Issues - A high number of open issues can indicate that a project is not
  being well maintained. If the total number of open and closed issues is very
  small then it can mean that people aren't really using the framework as they
  haven't been asking questions, suggesting features, finding bugs - maybe the
  project is perfect, but that's probably not the case!
- Closed Issues - A high number of closed issues means that the project is
  likely being well maintained (especially this early on with Deno, in older
  languages you can have a high number and the project be dead for some time!)
  and that the community is actively involved in raising queries, suggestions
  etc.

## 文档

当你希望使用一个新的模块，关键要得到轻松上手的文档。 什么一般是一个快速入门指南
和一些参考，让你看到示例代码是最有用的。 当你开始使用该框架更严重和复杂的应用程
序，这是然后用更全面的称赞，但容易搜索和浏览，API 文档（即记录参数和返回类型的故
障等）和使用情况导游集（其可以是示例代码本身！）。

| 骨架         | 入门实例 | 代码示例 | 指南 | 完整 API 文档 |
| ------------ | -------- | -------- | ---- | ------------- |
| Abc          | ✅       | ✅       | ✅   | ✅            |
| Ako          | ✅       | ❌       | ❌   | ❌            |
| Aqua         | ✅       | ❌       | ✅   | ❌            |
| Attain       | ✅       | ✅       | ✅   | ✅            |
| Denotrain    | ✅       | ✅       | ✅   | ❌            |
| Drash 🌟     | ✅       | ✅       | ✅   | ✅            |
| Fastro       | ✅       | ✅       | ✅   | ❌            |
| Fen          | ✅       | ✅       | ✅   | ❌            |
| http_wrapper | ✅       | ✅       | ✅   | ❌            |
| Oak          | ✅       | ✅       | ✅   | ✅            |
| Opine        | ✅       | ✅       | ✅   | ✅            |
| Pogo         | ✅       | ✅       | ✅   | ✅            |
| Servest      | ✅       | ✅       | ✅   | ✅            |
| Snowlight    | ✅       | ✅       | ✅   | ✅            |

Most of the reviewed frameworks have reasonable documentation. Only some fall
down in fully documenting their APIs, however with the likes of
https://doc.deno.land/ and the usage of TypeScript, it is still possible to
understand Deno modules reasonably well even if not documented extensively by
the authors.

A golden star shout-out to Drash which has some of the most amazing
documentation - it has everything from getting started, to full API docs,
diagrams of the internals and comprehensive tutorials in it's own dedicated
website.

## 性能

当涉及到生产服务器，负载下的性能是您的客户/客户提供良好的服务至关重要，并选择
Web 服务器框架时应该永远是一个考虑因素。 Of course there is the trade-off
between performance and rich features - generally frameworks that offer the
greatest set of features out-of-the-box are slightly heavier and thus slower
than those that are thin wrappers around the core / standard library. Always
take care to pick the server that offers the best performance for your current
and any potential future use-case.

To get a measure of performance I was going to write a set of benchmarks when I
happened across Fastro which supports a full benchmarking capability! So credit
must go @ynwd (currently the sole contributor) for the benchmark code!

> Note: This benchmark is not wholly realistic and you should always take care
> to benchmark and measure performance for your particular use-case.

For each framework I wrote the minimal amount of code in order to start a server
which would respond to a GET request to the root / path with a body of "Hello
Deno!". The performance was measured using the NPM package autocannon with
commands similar to:

```bash
npx autocannon -c100 -j localhost:3000
```

This was performed using:

- Machine: MacBook Pro, 2.3 GHz Intel Core i5, 8 GB 2133 MHz LPDDR3
- Node: 12.18.0
- Deno: 1.1.0

下面是结果，由每秒实现（从 Fastro 采集 PHP 和 Python Flask）平均请求排序方法：

| Framework    | Version        | Avg RPS  | Language |
| ------------ | -------------- | -------- | -------- |
| Deno HTTP    | 1.1.0 (0.57.0) | 20687.6  | Deno     |
| Node HTTP    | 12.18.0        | 19954.8  | Node     |
| Denotrain    | 0.5.2          | 19022    | Deno     |
| http_wrapper | 0.5.0          | 18172.8  | Deno     |
| Fastro       | 0.10.1         | 17808    | Deno     |
| Fastify      | 2.14.1         | 17538.55 | Node     |
| Drash        | 1.0.5          | 16305.2  | Deno     |
| Aqua         | master         | 16235.2  | Deno     |
| Abc          | 1.0.0-rc10     | 14316.4  | Deno     |
| Attain       | 0.9.4          | 14095.2  | Deno     |
| Oak          | 4.0.0          | 13851.2  | Deno     |
| Pogo         | 0.4.0          | 11137.6  | Deno     |
| Express      | 4.17.1         | 10747.2  | Node     |
| Fen          | 0.8.0          | 9265.21  | Deno     |
| Opine        | 0.8.0          | 8409.8   | Deno     |
| Snowlight    | master         | 8360     | Deno     |
| Servest      | 1.1.0          | 7452.8   | Deno     |
| Ako          | master         | 6329.1   | Deno     |
| PHP          | 7.3.11         | 6055.9   | PHP      |
| Python Flask | 1.1.2          | 528.21   | Python   |

有趣的是杰诺的标准 HTTP 竟是更快（在我的设置，在这个基准测试 - 结果可能会有所不
同！），比 LTS 节点 12.18.0 尽管杰诺基准找到节点的 HTTP 服务器性能通常会更快。这
可能是，即使我们已经添加了一丝的复杂性，我们正在观察这意味着在多个请求的当然是一
直快速杰诺的远优于尾延迟的影响，而节点可以是非常不稳定。

突出`Deno`模块`Denotrain`和`Fastro`其中两个支持路由器和中间件，并且合理地接近原
始`Deno` HTTP 库的速度。 `http_wrapper`也是搭配，如果你需要一个快速的路由器，但
不需要中间件结构。

## 熟悉

虽然你应该拥抱变化，并采用舒适的最佳实践为特定的语言，有时下车地面的最简单的方法
就是找到最当前正在使用库相匹配。这样，您就可以将现有的项目，以充分利用之类的东西
杰诺的增强的安全性，插件支持等以最小的开销，而你和你的团队都将能够开发和轻松地扩
展程序的 API 是熟悉的。

在这一节中，我试图找出这些库背后的灵感和相似性进行排名，以现有节点库:

### Express

1. Opine - 不仅是'Opine`启发`Express`, it is directly ported from it meaning
   both the API and internals are very similar (if not exactly) to Express.
   [Disclaimer: I am the author!]
2. Attain - Attain supports an Express like middleware and Router API, with a
   few differences. Each handler is passed an Oak Request object and a Response
   object which offers several of the Express response APIs.
3. Servest - Another Express inspired HTTP server framework, Servest has many
   similar APIs to Express, though some named slightly differently. Unlike
   Express it's built in request object has methods for the parsing of requests,
   and also supports request filtering in it's handler API. It also has a logger
   built-in which one must configure to prevent from logging every request at
   INFO level.
4. Snowlight - Snowlight is inspired by Express and much of it's API is taken
   directly from Express. It has some subtle differences such as the addition of
   a app.group() method allowing you to mount of shared middlewares onto a
   router for a specific path and lacks some of the less common response APIs
   etc.
5. http_wrapper - A minimal wrapper around the `Deno` HTTP standard library with
   a Router inspired by `Express`.
6. Aqua - 路由器和中间件松散影像`Express` API。

### Koa

1. [Oak] - `Oak`被`Koa`和支持激发了相当丰富的上下文驱动的中间件 API，反映`Koa`。
2. [Denotrain] - 尽管它指出该库由`Express`启发，我已经选择把它列`Koa`下部分，因
   为它的使用就像对象的上下文，而不是连接样式（REQ，水库，下一个）中间件 API 的
   。它不会从兴亚但不同之处在于响应从路由执行返回并没有`next（）'的中间件。
3. [Ako] - `Ako`目的端口`Koa`到`Deno`，因此可能最终成为谁愿意将他们的应用程序
   到`Deno`用户的最佳选择。然而在此刻已经非常有限的文档，所以它并不容易，以确定
   有多少是镜子兴亚 API - 我建议你一定认为它`Koa`像`Deno`应用，这可能是因为它计
   划简单地使用`Koa`文档，因为它的文档！

### Fastify

1. Fastro - Inspired by Express, Nest & Firebase, but mostly takes it's API from
   Fastify.

### Hapi

1.Pogo - Pogo is inspired by Hapi with a matching route object API and a rich
supporting API.

### Laravel

1. Drash - Drash provides middleware similar to Laravel, but also takes
   inspiration from Flask and Tonic, as well as introducing it's own concepts.

### 未分类

下面的框架并没有觉得他们很好地适应另一个的上述类别:

1. Abc - Although Abc API supports a context like object, it is somewhat
   different to the Koa context, and is unlike the other existing Node web
   frameworks.
2. Fen - Fen also supports a context like object API for it's routing and
   middleware, but makes use of a router and controller setup which differs
   somewhat from the listed Node modules.

### 那些逃走

正如你可能已经注意到，我没有查看所有可用的框架的 100％。的那些在这篇文章中重点是
，以保持合理的范围内，并限制其测试，审查和写了一切的开销基于以下考虑选择(!):

- 是否 README 使其明确，该项目是一项正在进行的工作？如果这样的话，不检讨。
- 是框架和抽象了一个框架？如果是这样，那就不要评论。
- Am I able to easily understand the documentation and get started? If not, then
  don't review. (This is just my opinion!) I encourage you to also review the
  following which based on these (relatively arbitrary!) factors and maybe some
  others I did not cover this time round:
- Alosaur - A very cool looking Decorator based web framework which appears to
  be well maintained and popular. One I plan to try out and review in the
  future.
- Levo - A frontend framework that supports Server-Side Rendering (SSR) and The
  Elm Architecture (TEA) out of the box. This framework is designed for web
  pages and SPAs, and shouldn't be used for APIs. It offers lots of awesome
  features out of the box like brotli compression, directory based routing and
  virtual DOM diffing - definitely one to watch!
- Dactyl - Dactyl is built on top of the Oak framework, and aims to achieve the
  same goals as Nest did for Express by providing declarative controllers to the
  user. Like Alosaur it makes heavy use of Decorators available with Deno's
  support of TypeScript. MandarineTS - MandarineTS is a typescript framework for
  creating websites using the Model View Controller (MV) pattern. It has several
  features such as built-in dependency injection, sessions, ORM and a templating
  engine, and also makes good use of Decorators.
- Dinja - Dinja, like Dactyl, is another higher level framework providing APIs
  over the top of the Pogo framework, and pulling in a templating engine, and
  various SQL modules.
- Mith - A middleware framework inspired by Express. Where Mith differs from
  Express is how it solely focusses on providing a robust middleware system, and
  thus has no support for Routes. It also attempts to leave the Deno
  ServerRequest and Response objects as untouched as possible.
- Jurassic - A path "zero-config", path based routing framework.
- Arkoren - Arkoren says it "aims to be one of the next generation web
  frameworks available" with features such as "type-safe" and APIs inspired by
  Laravel. It however is currently in early stages of development and thus is
  not ready for use.
- SF - I am truly baffled by this web framework offering. On one hand it makes a
  point of discussing "how stupid" it is, without really ever explaining it's
  purpose! It intrigues me because from the limited documentation it appears to
  support a whole host of features and APIs for dealing with Redis, CRON, making
  HTTP requests as well as handling and responding to HTTP(S) requests. Deno
  Express - A clone of one of the original demo servers for Deno, despite it's
  name there is no similarity in it's internals to Express, though it does offer
  an elegant minimal Express-like API. It's not clear whether it is being
  actively worked on?
- MiniServer - MiniServer is a very minimal server wrapper around the standard
  Deno library.
- Centroid - A work in progress project for implementing the MOST Web Framework
  to Deno.
- Denosaur - A simple web framework that is currently a work in progress.
- Espresso - A work in progress minimal web framework that appears to take
  inspiration from Koa, but with an ambitious roadmap including database
  integration, graphql and MVC support.
- Denovel - "A Deno Framework For Web Artisan", Denovel is another web framework
  drawing inspiration from Laravel. Unlike most frameworks, the author
  encourages you to clone the repository as a starting point, it doesn't appear
  to be a module you can import - core packages can be taken from the vendor
  directory however.
- DeliGenius - A lightweight middleware framework with Koa inspired API and very
  impressive performance - one to watch.

## 总结

I hope this review was helpful!

If you have been using one of the mentioned web frameworks for Deno, or maybe
something I haven't covered(!), I would love to hear what you think about it -
please drop your reviews and comments in the section below.

If you are a maintainer of a project and I've missed of your project, or you
feel I've mis-represented it then please drop a comment and we can look to add /
update the article!

Until next time, thanks for reading! 🦕

P.S. 寻找一种方式来测试你的 HTTP 服务器？ 为什么查不出来 SuperDeno 🎉

超级`Deno`站在雨中夜间 - 坚忍地面对黑暗的战斗是软件工程

[oak]: https://github.com/oakserver/oak
[denotrain]: https://github.com/Caesar2011/denotrain
[ako]: https://github.com/ako-deno/ako

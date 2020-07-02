---
title: "代理"
linkTitle: ""
weight: 43
type: "docs"
description: >
  Deno supports proxies for module downloads and the Web standard `fetch` API.
---

Proxy configuration is read from environmental variables: `HTTP_PROXY` and
`HTTPS_PROXY`.

In case of Windows, if environment variables are not found Deno falls back to
reading proxies from registry.

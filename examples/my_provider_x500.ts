#!/usr/bin/env mashin run
import * as myProvider from "../mod.ts";

// configure aws provider
const provider = new myProvider.Provider("sample_provider_name", {
  version: 2,
});

[...Array(500)].map((_, i) => {
  new myProvider.MyResource(
    `myresource${i}` as Lowercase<string>,
    {
      myKey: "hello!",
    },
    {
      provider: provider,
    }
  );
});

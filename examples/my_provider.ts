#!/usr/bin/env mashin run
import * as myProvider from "../mod.ts";

// configure aws provider
const provider = new myProvider.Provider("sample_provider_name", {
  version: 2,
});

new myProvider.MyResource(
  "myresource",
  {
    myKey: "hello!",
  },
  {
    provider: provider,
  }
);

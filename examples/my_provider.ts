#!/usr/bin/env mashin run
import * as myProvider from "../mod.ts";

// configure aws provider
const provider = new myProvider.Provider("sample_provider_name", {
  myKey: "hi",
});

const my_resouce = new myProvider.MyResource(
  "myresource",
  {
    myKey: "hello!",
  },
  {
    provider: provider,
  }
);

console.log(my_resouce);

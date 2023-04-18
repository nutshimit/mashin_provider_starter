/* -------------------------------------------------------- *\
*                                                          *
*      ███╗░░░███╗░█████╗░░██████╗██╗░░██╗██╗███╗░░██╗     *
*      ████╗░████║██╔══██╗██╔════╝██║░░██║██║████╗░██║     *
*      ██╔████╔██║███████║╚█████╗░███████║██║██╔██╗██║     *
*      ██║╚██╔╝██║██╔══██║░╚═══██╗██╔══██║██║██║╚████║     *
*      ██║░╚═╝░██║██║░░██║██████╔╝██║░░██║██║██║░╚███║     *
*      ╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═════╝░╚═╝░░╚═╝╚═╝╚═╝░░╚══╝     *
*                                         by Nutshimit     *
* -------------------------------------------------------- *
*                                                          *
*   This file is generated automatically by mashin.        *
*   Do not edit manually.                                  *
*                                                          *
\* ---------------------------------------------------------*/

import * as resource from "https://mashin.land/sdk/resource.ts";
import { Inputs, Outputs } from "https://mashin.land/sdk/output.ts";
import { getFileName } from "https://mashin.land/sdk/download.ts";

export const VERSION = "0.1.0";
const LOCAL_PATH = Deno.env.get("LOCAL_PLUGIN")
  ? "./target/debug/libmashin_provider_starter.dylib"
	: await globalThis.__mashin.downloadProvider(
      "github",
      new URL(
        getFileName("mashin_provider_starter"),
        `https://github.com/nutshimit/mashin_provider_starter/releases/download/v${VERSION}/`
      ).toString()
    );	

export interface MyResourceConfig extends Inputs {
  myKey: string | undefined | null;
}
export interface MyResourceOutputs extends Outputs {
  myKey: string | undefined | null;
  mySensitiveKey: string | undefined | null;
}

export class MyResource<T extends Lowercase<string>> extends resource.Resource<
	MyResourceOutputs,
	T
> {
    #props: MyResourceConfig;
    constructor(
        name: resource.ResourceName<T>,
        props: MyResourceConfig,
        opts: resource.ResourceOptions
    ) {
        super(name, "my_module:my_resource", props, opts);
        this.#props = props;
    }

    get props() {
        return this.#props;
    }
}

/**
  * This is my provider config
  **/
export interface Config extends Inputs {
/**
  * This is my key
  **/
  myKey: string | undefined | null;
}

export class Provider extends resource.Provider {
    constructor(name: string, args?: Config) {
      // FIXME: have dynamic provider path for each OS
      super(name, LOCAL_PATH, args);
    }
}

// 15819527346317364072
# Mashin Provider Starter

The Mashin Provider Starter is a Rust-based starter kit that provides a simple, clean foundation for building provider plugins for the Mashin platform. This project is designed to help developers get up and running quickly with the Mashin SDK, so that they can start building high-quality providers that integrate with the engine.

This project is currently **under active development** and is intended to be a proof of concept. As such, it may change at any time and should not be used in production environments.

To get started, simply clone the repository using the following command:

```bash
git clone https://github.com/nutshimit/mashin-provider-starter
```

If you have any questions or concerns about the mod.ts file or its relationship to the src/lib.rs file, please consult the documentation or reach out to the project maintainers for assistance.

Once you've cloned the repository, you can open the `src/lib.rs` file to start building your provider plugin. This file provides a basic implementation of the provider plugin API, along with some sample code that demonstrates how to interact with the Mashin platform.

The SDK is designed to be easy to use and flexible, so that you can customize it to meet your specific needs. You can add additional functionality to your provider by leveraging Rust's powerful libraries and frameworks, or by implementing custom logic that integrates with other tools in your workflow.

## Typescript bindings

Please note that the `mod.ts` file included in this project is generated automatically from the struct defined in the `src/lib.rs` file. As such, it should not be edited directly. Instead, any changes you make to the struct in the code, should be automatically reflected in the `mod.ts` file.

This separation of concerns helps to ensure that the project remains easy to maintain and consistent across different components. It also helps to reduce the risk of errors or conflicts that might arise from manual editing of the mod.ts file.
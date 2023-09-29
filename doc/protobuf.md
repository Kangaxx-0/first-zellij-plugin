# Zellij plugin Api

In the [wasm introduction](./wasm.md) section, we touched based upon the use of Protocol Buffers(protobuf) in zellij for facilitating data transfer between the plugin and the host. In this document, we delve deeper into  the architecture and specifics of the protobuf API in Zellij.

> This doc is based on the following [commit](https://github.com/zellij-org/zellij/commit/7ccefc0d6ca6bcc079dcbf24f64ec1368d1b3791)

## Overview

For each `.proto` file in zellij codebase, there exist a corresponding rust file responsible for conversions between protocol buffer and native rust types.

## Components

Here are the major components of the API, which are defined in .proto files:

### Action 
TBD

### Message
This component represents asynchronous messages intended for specific workers to handle. Typically, it's used for long-run job.

### Event
The event is crucial for host-to-plugin communication. Whenever the host needs to inform the plugin about any changes or updates, it triggers an event which the plugin listens to.

###  Plugin_command
The plugin_command is arguably one of the most critical .proto files. It defines the APIs that the plugin can call. Below are the files that are related, either directly or indirectly, to plugin_command: 
- Plugin Permission: Manages the authorization level for different plugins.
- Command: Contains definitions for specific operations that can be performed.
- File: Includes functionalities related to file operations.
- Input Mode: Specifies the various input modes supported by the plugin.
- Key: Deals with key mappings and keyboard interactions.
- Plugin IDs: Contains identifiers for each plugin for mapping and tracking.
- Resize: Provides details on how resizing events should be handled.
- Style: Holds information on styling and theming options for the plugin UI.

[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/ishche/dapkit) 

# Debug adepter protocol toolkit

The dapkit (Debug adapter protocol toolkit) is a tool to emulate a debug adapter of ide.

The plan is to support two use cases.
1. dapkit as proxy listen to all communication between ide and debug adapter. All messages then stored in a script file.
2. dap can play the script file and act as a debug adapter or ide.

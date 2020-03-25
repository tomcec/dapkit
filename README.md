[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/ishche/dapkit) 

# DAPKit

DAPKit (Debug Adapter Protocol toolKit) is a tool to proxy or mock 
[DAP](https://microsoft.github.io/debug-adapter-protocol/specification) communication between 
an IDE and a debug adapter.

![DAPKit place diagram](https://raw.githubusercontent.com/ishche/dapkit/master/dia-place.png)

DAPKit is useful for IDE and debug adapter developers if they work with debag adapter protocol.

The application can be useful for test automation as IDE or debag adapter mock. At the same time, 
DAPKit can be a quick way to create a mock of debug adapter to share with third parties.  It can 
be useful for bug reproducing without sharing the actual debug adapter and target system.

![IDE mock](https://raw.githubusercontent.com/ishche/dapkit/master/dia-mock-ide.png)

![DA mock](https://raw.githubusercontent.com/ishche/dapkit/master/dia-mock-da.png)

In proxy mode, DAPKit can on fly detect errors in DAP communication. This can reduce development and 
testing time and increase debug adapter protocol compliance of debug adapter.

![Script validation](https://raw.githubusercontent.com/ishche/dapkit/master/dia-validate-script.png)

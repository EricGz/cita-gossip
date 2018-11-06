# CITA-Gossip

test

A project which use Gossip protocol with a modern TLS library. Intending to accomplish synchronization in clusters and serves Data transfer related to Block Chain. Cooperation with CITA developed by Cryptape.

The language [*rust*](https://www.rust-lang.org/zh-CN/) is used.

---
## Overview

### Project intro

 - Gossip协议在异步系统中被广泛应用于消息的同步和处理。其在网络资源占用和性能上的表现较为均衡，能够很好的满足P2P网络结构下的网络消息散播、状态同步。
 - 本项目旨在利用Gossip的消息同步方式，基于TCP+TLS完善的底层通信，实现集群状态的同步，同时实现对更上层CITA应用的网络传输接口。
 - 设计并引入节点发现协议, 动态发现周围节点。

### Project structure

The project is constructed with several mods. Each of them holds a part of the function of gossip.

A sketch map:

![logo](https://github.com/shelgor/cita-gossip/blob/master/docs/module%20design.png) 

Introduction:
* comm: 
    - Transfer module with TCP+TLS, involves [*rustls*](https://github.com/ctz/rustls)
* discovery: 
    - Node self discovey module
    - using configuration methods & mdns(module storage) methods
* filter:
    - contains the main logic controll of gossip protocol
    - randomly send & periodically send
* msg: 
    - define some data types, functions and interfaces which builds connections between modules
* mDNS:
    - mDNS protocol using for new node's self declaration of being alive
* storage:
    - operations of the stored list file. The list records the status of the cluster, mainly focus on those who is alive.

---
## Status
The design is complete.Small adjustments are being considered.
We have built up the whole structure of the project. 
It's currently in development and hence unstable.
 - What are we doing now:
     - Testing modules.
     - CA method's improvent.
     - The interface between module "comm" and module "msg", here are some message parsing problems.
     - How to solve the three kinds of messages that could be received.

---

## Features

### Current Features

 - New nodes can join in the cluster.
 - Acknowledge and encrypted data transfer between nodes.
 - Random send which is the push method of gossip.
 - 
### Possible Features

 - Unicast in application layer
 - Read-only nodes not in requirement of applying new CA.

---

## Instructions and Approaches
Here introduces how to use and run this project. Due to some functions and modules are under developing, it may not work well for the moment.
 - You have to get a complete set of certifications, public and private keys to run module comm. The shell script in /test-ca helps you get one that could be used to verify the function of comm.
 - 

---
## Reference
module comm reference [*rustls*](https://github.com/ctz/rustls)
module mDNS reference [*rustls*](https://github.com/ctz/rustls)

---
## License

Rustls is distributed under the following three licenses:

 - Apache License version 2.0.
 - MIT license.
 - ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC respectively. You may use this software under the terms of any of these licenses, at your option.

---

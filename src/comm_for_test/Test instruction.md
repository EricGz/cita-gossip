# "comm" inroduction & Test instruction

This the instruction of how to test module comm.

The module comm involves files from keys and certs to rust programs. 

It is currently under development, so please give us feedback if any bug has been noticed.

## comm intro

-temporarily written in CN

相关文件和路径：
 
 - src：包含rustls的源文件
 - test-ca ： 密钥、证书的生成和保存
 - examples：包含客户端、服务端的rs文件
 
 其他文件夹和文件保留自rustls，确认无需后逐步剔除

## Test instruction
 
-temporarily written in CN

 - 运行test-ca中的build-a-pki.sh，openssl.cnf是其配置文件，生成 /rsa 和 /ecdsa，分别包含两种加密方式的三层证书
 - 在包含Cargo.toml的目录下运行cargo build
 - examples文件夹中的tlsclient.rs和tlsserver.rs为可执行测试的客户端和服务端；client.rs和server.rs为修改后为msg模块提供接口，此接口正在调整。
 - 编译tlsclient.rs和tlsserver.rs生成可执行文件
 - 命令行执行可执行文件

 测试tlsclient，可以直接使用其http模式访问网站验证功能
 ```
 比如在可执行文件的目录下
 
$ tlsclient --http www.baidu.com

可以接收到返回的html报文
```

测试tlsserver功能时
```
tlsserver --certs test-ca/rsa/end.fullchain --key test-ca/rsa/end.rsa -p 8443 echo

用echo模式可以测试其能否接收到client端发来的echo消息

```

## features under developing
 - 通过msg模块调用client的方式运行正式版client
 - server的报文解析
 - 按照建议学习如何用tokio替代正在使用的mio，重新实现异步处理的部分，降低代码的重复性

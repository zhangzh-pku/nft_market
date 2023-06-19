# NFT Market

NFT Market是一个基于Rust编写的非同质化代币（NFT）市场库。它使用了[OpenZeppelin](https://openzeppelin.com/)的智能合约库来实现NFT的相关操作。

## 主要组件和功能

### config

`config.rs`文件定义了一个名为`Config`的结构，用于处理库的配置信息。`Config`结构有以下字段：`infura_apikey`, `contract_address`, `account_address`, `private_key`。

`Config`结构提供了以下方法：

- `from_file(path: &str)`: 从指定路径的文件中读取配置信息。
- `get_infura_apikey()`: 返回Infura API的密钥。
- `get_contract_address()`: 返回NFT合约的地址。
- `get_instance()`: 返回`Config`实例的引用。
- `get_my_account()`: 返回用户账户的地址。
- `get_my_private_key()`: 返回用户账户的私钥。

### eth

`eth.rs`文件包含了与以太坊交互的函数。以下是主要的函数：

- `get_balance(address: &str)`: 返回指定地址的NFT余额。
- `mint()`: 创建新的NFT。它需要以下参数：`contract_address`, `user_address`, `my_account`, `my_private_key`, `token_uri`, `amount`。

### http

`http.rs`文件处理HTTP请求。主要的函数包括：

- `nft_balance(address: String)`: 返回指定地址的NFT余额。
- `mint()`: 创建新的NFT。需要以下参数：`contract_address`, `user_address`, `token_uri`, `amount`.

`run_server()`函数启动HTTP服务器，处理来自客户端的请求。

## 主函数

在`main.rs`文件中，`main()`函数首先从`config.json`文件中读取配置信息，然后运行HTTP服务器。

## 依赖关系

此库依赖于OpenZeppelin的智能合约库。


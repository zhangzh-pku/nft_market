NFT Market
NFT Market是一个基于Rust编写的非同质化代币（NFT）市场库。它使用了OpenZeppelin的智能合约库来实现NFT的相关操作​1​。

主要组件和功能
以下是库的主要组件和它们的功能：

config
config.rs文件定义了一个名为Config的结构，用于处理库的配置信息。Config结构有以下字段：

infura_apikey: Infura API的密钥。
contract_address: NFT合约的地址。
account_address: 用户账户的地址。
private_key: 用户账户的私钥。
Config结构提供了以下方法：

from_file(path: &str): 从指定路径的文件中读取配置信息。
get_infura_apikey(): 返回Infura API的密钥。
get_contract_address(): 返回NFT合约的地址。
get_instance(): 返回Config实例的引用。
get_my_account(): 返回用户账户的地址。
get_my_private_key(): 返回用户账户的私钥​2​。
eth
eth.rs文件包含了与以太坊交互的函数。以下是主要的函数：

get_balance(address: &str): 返回指定地址的NFT余额。
mint(): 创建新的NFT。它需要以下参数：
contract_address: NFT合约的地址。
user_address: 用户账户的地址。
my_account: 用户账户的地址。
my_private_key: 用户账户的私钥。
token_uri: 新NFT的URI。
amount: 新NFT的数量​3​。
http
http.rs文件处理HTTP请求。主要的函数包括：

nft_balance(address: String): 返回指定地址的NFT余额。
mint(): 创建新的NFT。需要以下参数：
contract_address: NFT合约的地址。
user_address: 用户账户的地址。
token_uri: 新NFT的URI。
amount: 新NFT的数量。
run_server()函数启动HTTP服务器，处理来自客户端的请求​4​。

主函数
在main.rs文件中，main()函数首先从config.json文件中读取配置信息，然后运行HTTP服务器​5​。

依赖关系
此库依赖于OpenZeppelin的智能合约库​1​。
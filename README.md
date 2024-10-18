
# pbft+dagblock
使用rust实现pbft+dagblock的双层网络；dag网络数据达成共识后，传输到pbft网络进行再次共识。

# 启动程序
启动4个共识节点，4个节点的dag网络
```bash
cargo run -- -n 4 -f 4
```
-n 代表4个共识节点； -f 代表4个节点的dag网络

# 测试

## 测试所有用例
测试dag所有用例，/blockdag/src/lib中tests模块
```bash
cargo test -p blockdag  -- --nocapture
```
## 测试单独用例
测试单独用例，/blockdag/src/lib中tests模块，test_fig3可以切换为想要的测试名字


```bash
cargo test -p blockdag  tests::test_fig3 -- --nocapture
```
<img src="../imgs/Fig.3.png" width="800px"/>

```bash
cargo test -p blockdag  tests::test_fig4 -- --nocapture
```
<img src="../imgs/Fig.4.png" width="800px"/>



# 日志
不同类型的日志：
- ⛏️：dag网络挖矿
- 💻: 客户端
- 😃: 正常节点
- 😈: 异常节点
- 🌟:  代表 PrePrepare 
- 🌟🌟: 代表  Prepare 
- 🌟🌟🌟: 代表  Commit 
- ✅: 表示客户端已经收到f+1个相同的回复，并且已经达成共识


# 注释
## 框架



## 文件说明
没有写注释的，可以忽略了
blockdag/src/blockdag      
    anticone.rs  anticone的相关计算函数  
    block.rs  block相关操作，例如，获取分数最高的block，等；  
    calcblue.rs  计算k-cluster，这里成为blue，不属于k-cluster的成为red；  
    cardinality.rs  计算分数的辅助函数，  
    dagsim.rs  向dag中添加block  
    node.rs  节点的相关操作，例如，处理区块，添加区块等  
blockdag/src/  
    lib.rs  启动dag网络的所有节点，进行blockdag共识。  
    sync.rs  向pbft网络发送数据，启动pbft共识  

src/consensus  
    message.rs  pbft网络信息类型  
    pbft.rs  pbft状态  
src/network  
   client.rs  client的定义，以及功能实现
   launcher.rs  节点启动
   node.rs  节点定义  
   server.rs  pbft功能实现
   utils.rs  辅助函数  

src/main.rs  程序启动文件  


















# pbft+dagblock
使用rust实现pbft+dagblock的双层网络；dag网络数据达成共识后，传输到pbft网络进行再次共识。

# rust环境
[rust环境安装](https://www.rust-lang.org/zh-CN/tools/install)
[rust教程](https://www.runoob.com/rust/rust-tutorial.html)

# 启动程序
启动4个共识节点，4个节点的dag网络
```bash
cargo run -- -n 4 -f 4
```
-n 代表4个共识节点； -f 代表4个节点的dag网络

# 测试数据

## 测试所有用例
测试dag所有用例，/blockdag/src/lib中tests模块
```bash
cargo test -p blockdag  -- --nocapture
```
## 测试单独用例
测试单独用例，/blockdag/src/lib中tests模块，test_fig1可以切换为想要的测试名字
```bash
cargo test -p blockdag  tests::test_fig1 -- --nocapture
```
## blockdag的k-cluster的测试

```bash
cargo test -p blockdag  tests::test_fig1 -- --nocapture
```
<img src="../imgs/Fig.1.jpg" width="800px"/>

## dag网络性能测试
调节参数  
`let blocks_one_time: i32 = 4;` 数字越大，性能越高；

```bash
cargo test -p blockdag  tests::test_nodes_performance -- --nocapture
```
`let blocks_one_time: i32 = 4;`结果如下，耗时2586ms，区块1000个
```
running 1 test

preparing to terminate. wait 1 second for nodes complete propagation....

========================test done.==================================
test_nodes_performance(): start. k=3, blocks=1000, nodes=4
total time used: 2586 (ms)
test tests::test_nodes_performance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 3.69s
```

`let blocks_one_time: i32 = 10;`结果如下，耗时1040ms，区块1000个
```
running 1 test

preparing to terminate. wait 1 second for nodes complete propagation....

========================test done.==================================
test_nodes_performance(): start. k=3, blocks=1000, nodes=4
total time used: 1040 (ms)
test tests::test_nodes_performance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 2.14s
```

# 对比数据
## pbft-dag vs kaspa
|  性能（tps）| pbft+dag | kaspa     | 
| ---------- | -------- | --------- | 
| 1          | 387      | 181.7     | //实测值
| 2          | 1000     | 300       | //理论值

`let blocks_one_time: i32 = 4;`耗时2586ms，区块1000个 `1000/2.58=387`

Kaspa 是基于 GHOSTDAG协议构建的POW公链，Kaspa提出了PHANTOM协议，这是一种基于工作量证明的无许可分类账协议，它将中本聪所定义区块链推广至有向无环图(blockDAG)。PHANTOM可以引用多个前区块，提供所有区块和交易的总排序，并输出一组一致的已被接受的交易。当前kaspa的技术原理主要在其2021年发表的论文《PHANTOM GHOSTDAG：A Scalable Generalization of Nakamoto Consensus》
根据kaspa浏览器，实际数据，10-22，`15702645/(24*60*60) = 181.7`
<img src="../imgs/Fig.2.png" width="800px"/>

## pbft-dag vs pbft-pbft
|   出块时间  | pbft+dag |pbft+pbft    |
| ---------- | -------- |------------- |
| 1          | 3s       |7s            |  //共识时间
|            |          |              |


pbft-dag: `1729823185 - 1729823182 = 3`
```
🎮  >>>>>>>>>>>> start test timestamp:1729823182!!!
... ...
[💻 Client] Received ReplyMsg: Json(ReplyMsg { time_stamp: 1729823185, view_id: 9999, node_id: 1, client_id: 0, result: "new block !!" })
🎮  <<<<<<<<<<<<<<<<<< end test timestamp:1729823185!!!
```

pbft-pbft: `1729822892 - 1729822885 = 7`
```
[💻 Client] Received RequestMsg: Json(RequestMsg { operation: "new block !!", time_stamp: 1729822885, client_id: 0, sequence_id: 0, digest: "" })
🎮  >>>>>>>>>>>> start test timestamp:1729822885!!!
... ...
✅  Client received f+1 identical replies, consensus reached: new block !!
🎮  <<<<<<<<<<<<<<<<<< end test timestamp:1729822892!!!
```




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

## 文件说明
没有写注释的，可以忽略了

blockdag/src/blockdag      dag网络相关  
    anticone.rs  anticone的相关计算函数  
    block.rs  block相关操作，例如，获取分数最高的block，等；  
    calcblue.rs  计算k-cluster，这里成为blue，不属于k-cluster的成为red；  
    cardinality.rs  计算分数的辅助函数，  
    dagsim.rs  向dag中添加block  
    node.rs  节点的相关操作，例如，处理区块，添加区块等  
blockdag/src/  
    lib.rs  启动dag网络的所有节点，进行blockdag共识。  
    sync.rs  向pbft网络发送数据，启动pbft共识  

src  pbft相关  
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

















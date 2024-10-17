
# pbft+dagblock
使用rust实现pbft+dagblock的双层网络；dag网络数据达成共识后，传输到pbft网络进行再次共识。

# 启动程序
启动4个共识节点，4个普通节点的dag网络
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
cargo test -p blockdag  tests::test_fig_x2 -- --nocapture
```


# 日志
不同类型的日志：
- 💻: 客户端
- 😃: 正常节点
- 😈: 异常节点
- 🌟:  代表 PrePrepare 
- 🌟🌟: 代表  Prepare 
- 🌟🌟🌟: 代表  Commit 
- ✅: 表示客户端已经收到f+1个相同的回复，并且已经达成共识


# 注释
## 框架



## 目录结构
没有写注释的，可以忽略了
src/consensus
    message.rs
    pbft.rs
src/network
   client.rs
   launcher.rs
   node.rs
   server.rs
   utils.rs
src/main.rs

blockdag/src/blockdag  
    anticone.rs
    block.rs
    calcblue.rs
    cardinality.rs
    dagsim.rs
    node.rs
blockdag/src/
    lib.rs
    sync.rs















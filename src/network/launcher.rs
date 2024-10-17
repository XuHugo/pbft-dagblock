use crate::network::client::{start_new_block, Client};
use crate::network::node::Node;
use crate::network::server::Server;
use daggy::{Dag, NodeIndex, Walker};
use std::{io, time};

use super::serverd::ServerD;

#[derive(Copy, Clone, Debug)]
struct Edge;

pub fn start_nodes_pbft(n: u32) -> io::Result<()> {
    let mut dag = Dag::<Node, Edge>::new();
    let mut root_0 = NodeIndex::new(0);

    let mut servers = Vec::new();
    //let mut serverds = Vec::new();

    //consensus network
    for i in 0..n {
        let is_faulty = false;
        let port = 8000 + i;
        let mut server = Server::new(i, port as u16, n, is_faulty);
        if i == 0 {
            root_0 = dag.add_node(server.node.clone());
        }
        server.start();
        servers.push(server);
    }

    let mut client = Client::new(n);
    client.start();
    //start_new_block();

    // wait for all server threads (servers will run indefinitely)
    // for server in servers {
    //     server.join();
    // }

    Ok(())
}

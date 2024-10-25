pub mod network {
    pub mod client;
    pub mod launcher;
    pub mod node;
    pub mod server;
    pub mod serverd;
    mod utils;
}

mod consensus {
    pub(crate) mod message;
    pub(crate) mod pbft;
}

#[cfg(test)]
mod tests {
    use crate::network::client::{start_new_block, Client};
    use crate::network::launcher;
    use crate::network::node::Node;
    use crate::network::server::Server;
    use daggy::{Dag, NodeIndex, Walker};
    use std::{io, time};

    use crate::network::client::test_new_block_for_doublepbft;
    struct Edge;
    #[test]
    fn test_times() {
        assert_eq!(2 + 2, 4);
    }
}

use serde::{Deserialize, Serialize};
use std::{io, thread, time};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestMsg {
    //<REQUEST, o, t, c>
    pub(crate) operation: String, // 'o', the operation to be executed
    pub(crate) time_stamp: u64,   // 't', the time stamp
    pub(crate) client_id: u32,    // 'c', the client id
    pub(crate) sequence_id: u32,  // 'n', the sequence number
    #[serde(skip)]
    pub(crate) digest: String, // compute&save digest when receiving the request message for performance consideration
}

pub fn start_new_block(id: u32) {
    let _ = thread::spawn(move || {
        let mut id = 0;
        let ten_s = time::Duration::from_secs(10);
        //thread::sleep(ten_s);
        print!(
            ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>new block:{:?}",
            id
        );
        if let Err(e) = start_consensus(id) {
            eprintln!("Client failed to new block : {}", e);
        }

        //    id = id + 1;
    });
}
pub fn start_consensus(id: u32) -> io::Result<()> {
    let client = reqwest::Client::new();
    let time_stamp = chrono::Local::now();

    let request_msg = RequestMsg {
        operation: String::from("new block !!"),
        time_stamp: time_stamp.timestamp() as u64,
        client_id: 0,
        sequence_id: id,
        digest: String::new(),
    };

    actix_web::rt::System::new().block_on(async move {
        match client
            .post("http://127.0.0.1:9000/req")
            .json(&request_msg)
            .send()
            .await
        {
            Ok(response) => {
                // Handle the response here if needed
                println!("  -- Response from client {:?}", response.status());
            }
            Err(e) => {
                eprintln!(" -- Error sending request to client {}", e);
            }
        }
        return Ok(());
    })
}

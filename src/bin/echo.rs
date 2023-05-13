use anyhow::bail;
use serde::{Deserialize, Serialize};
use mael::protocol::*;

#[derive(Serialize, Deserialize)]
#[serde(tag="type")]
#[serde(rename_all="snake_case")]
enum EchoPayload {
    Echo {echo: String},
    EchoOk {echo: String},
}

struct EchoNode {
    _id: String,
    counter: usize,
}

impl Node<EchoPayload> for EchoNode {
    fn from_init(init: Init) -> Self {
        EchoNode { _id: init.node_id, counter: 0}
    }

    fn step(&mut self, msg: Message<EchoPayload>, output: &mut std::io::StdoutLock) -> anyhow::Result<()> {
        let mut reply = msg.into_reply(Some(&mut self.counter));
        
        match reply.body.payload {
            EchoPayload::Echo { echo } => {
                reply.body.payload = EchoPayload::EchoOk { echo };
                reply.send_message(output)?;
            },
            EchoPayload::EchoOk { echo } => bail!("unexpected message EchoOk: {echo}")
        }
        
        
        Ok(())
    }
}

fn main() -> anyhow::Result<()>{

    serve::<EchoNode, _>()?;
    
    Ok(())
}


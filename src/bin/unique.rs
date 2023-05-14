use std::io::StdoutLock;

use anyhow::bail;
use mael::protocol::{Node, Message, Init, serve};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UniqueIdPayload {
    Generate,
    GenerateOk{
        id: String
    }
}

struct UniqueIdNode {
    id: String,
    counter: usize,
}

impl Node<UniqueIdPayload> for UniqueIdNode {
    fn from_init(init: Init) -> Self {
        Self { id: init.node_id, counter: 0 }
    }

    fn step(&mut self, msg: Message<UniqueIdPayload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let mut reply = msg.into_reply(Some(&mut self.counter));
        
        match reply.body.payload {
            UniqueIdPayload::Generate  => {
                reply.body.payload = UniqueIdPayload::GenerateOk { id: format!("{}:{}", self.id, self.counter) };
                reply.send_message(output)?;
            }
            UniqueIdPayload::GenerateOk { .. } => bail!("unexpected message GenerateOk"),
        }
        
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    serve::<UniqueIdNode, _>()?;

    Ok(())
}



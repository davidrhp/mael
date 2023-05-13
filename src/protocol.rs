use std::io::{BufRead, StdoutLock, Write, StdinLock, Lines};

use anyhow::{bail, Context};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}

impl<Payload> Message<Payload>
where
    Payload: Serialize,
{
    pub fn into_reply(self, id: Option<&mut usize>) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
            body: Body {
                msg_id: id.map(|id| {
                    *id += 1;
                    *id
                }),
                in_reply_to: self.body.msg_id,
                payload: self.body.payload,
            },
        }
    }

    pub fn send_message(self, output: &mut StdoutLock) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *output, &self)?;
        output.write_all(b"\n")?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Body<Payload> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<Payload>
where
    Payload: Serialize,
{
    fn from_init(init: Init) -> Self;

    fn step(&mut self, msg: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn serve<N, Payload>() -> anyhow::Result<()>
where
    N: Node<Payload>,
    Payload: DeserializeOwned + Serialize,
{
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();
    
    let init_msg: Message<InitPayload> = read_init_message(&mut stdin)?;
    let init = extract_init(&init_msg)?;
    
    send_init_ok(init_msg, &mut stdout)?;

    let mut node: N = Node::from_init(init);
    for line in stdin {
        let msg = serde_json::from_str::<Message<Payload>>(&line?)?;
        node.step(msg, &mut stdout)?;
    }

    Ok(())
}

fn read_init_message(input: &mut Lines<StdinLock>) -> anyhow::Result<Message<InitPayload>> {
    let init_msg: Message<InitPayload> = serde_json::from_str(
        &input
            .next()
            .expect("no init message received")
            .context("failed to read init message from STDIN")?,
    )?;
    
    Ok(init_msg)
}

fn extract_init(msg: &Message<InitPayload>) -> anyhow::Result<Init> {
    let InitPayload::Init(init) = &msg.body.payload else {
        bail!("first message must be init")
    };
    
    Ok(init.clone())
}

fn send_init_ok(mut msg: Message<InitPayload>, output: &mut StdoutLock) -> anyhow::Result<()> {
    msg.body.payload = InitPayload::InitOk;
    msg.into_reply(None).send_message(output)?;
    Ok(())
}
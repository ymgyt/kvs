use crate::common::Result;
use crate::protocol::message::{MessageFrames, MessageType, Parse};
use crate::protocol::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Success {
    value: Option<Value>,
}

impl Success {
    pub(crate) fn new() -> Self {
        Self { value: None }
    }

    pub(crate) fn with_value(value: Value) -> Self {
        Self { value: Some(value) }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> Result<Self> {
        let value = parse.next_bytes_or_null()?.and_then(|v| Value::new(v).ok());

        parse.expect_consumed()?;

        Ok(Self { value })
    }

    pub(crate) fn take_value(&mut self) -> Option<Value> {
        self.value.take()
    }

    pub(crate) fn value(self) -> Option<Value> {
        self.value
    }
}

impl Into<MessageFrames> for Success {
    fn into(self) -> MessageFrames {
        let mut frames = MessageFrames::with_capacity(MessageType::Success, 1);

        match self.value {
            Some(value) => frames.push_bytes(value.into_boxed_bytes()),
            None => frames.push_null(),
        }

        frames
    }
}

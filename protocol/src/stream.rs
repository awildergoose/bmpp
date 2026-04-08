use crate::xml_args;

#[derive(Debug, Clone)]
pub enum StreamCommand {
    Open {
        from: String,
        to: String,
        version: String,
    },
    Unknown {
        text: String,
    },
}

impl StreamCommand {
    #[must_use]
    pub fn build(&self) -> String {
        match self {
            Self::Open { from, to, version } => format!(
                "<?xml version='1.0'?>{}",
                xml_args!("stream:stream", from, to, version)
                    .with_field("xmlns", "jabber:client")
                    .with_field("xmlns:stream", "http://etherx.jabber.org/streams")
                    .build()
            ),
            Self::Unknown { text } => text.clone(),
        }
    }
}

pub struct Stream {
    pipe: String,
}

impl Stream {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pipe: String::new(),
        }
    }

    pub fn feed(&mut self, command: &StreamCommand) {
        self.pipe += &command.build();
        println!("yum {command:?} => {}", command.build());
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}

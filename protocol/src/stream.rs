use crate::{xml::sanitize_text, xml_args};

#[derive(Debug, Clone)]
pub enum StreamCommand {
    XmlVersion {
        version: String,
    },
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
            Self::XmlVersion { version } => format!("<?xml version='{}'?>", sanitize_text(version)),
            Self::Open { from, to, version } => xml_args!("stream:stream", from, to, version)
                .with_field("xmlns", "jabber:client")
                .with_field("xmlns:stream", "http://etherx.jabber.org/streams")
                .build(),
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

    pub fn consume(&mut self) -> String {
        let pipe = self.pipe.clone();
        self.pipe = String::new();
        pipe
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}

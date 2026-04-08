#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod stream;
pub mod xml;

#[cfg(test)]
mod test {
    use crate::stream::StreamCommand;

    #[test]
    pub fn test_xml_open() {
        assert_eq!(StreamCommand::Open {
            from: "Joe".to_owned(),
            to: "Mama".to_owned(),
            version: "1.0".to_owned(),
        }.build(), "<stream:stream from='Joe' to='Mama' version='1.0' xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams'>");
    }

    #[test]
    pub fn test_xml_sanitization() {
        assert_eq!(StreamCommand::Open {
            from: "\'Joe".to_string(),
            to: "Mama".to_string(),
            version: "1.0".to_string()
        }.build(), "<stream:stream from='\\'Joe' to='Mama' version='1.0' xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams'>");
    }

    #[test]
    pub fn test_xml_unknown() {
        assert_eq!(
            StreamCommand::Unknown {
                text: "<something to='joe' content='hey'>".to_string()
            }
            .build(),
            "<something to='joe' content='hey'>"
        );
    }
}

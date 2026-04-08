use xml::{writer::events::StartElementBuilder, EmitterConfig};

#[must_use]
pub fn sanitize_text(text: &str) -> String {
    // TODO: definitely missing a lot here
    text.replace('"', "\\\"")
}

/// Converts a [`StartElementBuilder`] from [`xml`] to a [`String`]
///
/// # Panics
///
/// Panics if the writer fails or if the final [`String`] is non UTF-8
#[must_use]
pub fn build_xml(element: StartElementBuilder) -> String {
    let mut sink = Vec::new();
    {
        let mut writer = EmitterConfig::new()
            .write_document_declaration(false)
            .create_writer(&mut sink);
        writer
            .write(element)
            .expect("writer failed to write element");
    }
    String::from_utf8(sink).expect("writer produced non utf-8")
}

#[macro_export]
macro_rules! xml {
    ($name:expr, $($aname:expr, $avalue: expr),*) => {
        xml::writer::XmlEvent::start_element($name)
        $(
            .attr($aname, $avalue)
        )*
    };
}

#[macro_export]
macro_rules! xml_args {
    ($name: expr, $($kv: expr),*) => {
        $crate::xml!($name,
            $(
                stringify!($kv), $kv
            ),*
        )
    };
}

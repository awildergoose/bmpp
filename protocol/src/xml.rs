pub enum XmlComponent {
    Pair { key: String, value: String },
}

impl XmlComponent {
    #[must_use]
    pub fn build(&self) -> String {
        match self {
            Self::Pair { key, value } => {
                let value = value.replace('\'', "\\'");
                format!("{key}='{value}'")
            }
        }
    }
}

pub struct XmlBuilder {
    name: String,
    components: Vec<XmlComponent>,
}

#[macro_export]
macro_rules! xml {
    ($name: expr, $($key: expr, $value: expr),*) => {
        $crate::xml::XmlBuilder::new($name)
        $(
            .with_field($key, $value)
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

impl XmlBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            components: vec![],
        }
    }

    pub fn with_name(&mut self, name: impl AsRef<str>) -> &mut Self {
        self.name = name.as_ref().to_string();
        self
    }

    pub fn with_field(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        self.components.push(XmlComponent::Pair {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
        });
        self
    }

    #[must_use]
    pub fn build(&self) -> String {
        format!(
            "<{} {}>",
            self.name,
            self.components
                .iter()
                .map(XmlComponent::build)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

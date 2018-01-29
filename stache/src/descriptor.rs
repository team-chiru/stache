use expr::{ Description };
use toml;

#[derive(Default)]
pub struct Descriptor {
    descr: Option<Description>
}

impl Descriptor {
    pub fn from_description(descr: Description) -> Self {
        Self {
            descr: Some(descr)
        }
    }

    pub fn from_toml(raw_path: &str) -> Self {
        let descr: Option<Description>;

        match toml::from_str(raw_path) {
            Ok(d) => {
                descr = Some(d);
            },
            Err(e) => panic!("{:?}", e)
        }

        Self {
            descr: descr
        }
    }

    pub fn get(self) -> Option<Description> {
        self.descr
    }
}
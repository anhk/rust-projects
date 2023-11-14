pub struct Url {
    pub domain: String,
    pub port: u16,
    pub path: String,
    pub protocol: String,
}

impl Url {
    pub fn new(url: &String) -> Self {
        let mut u = Url {
            domain: String::from(""),
            port: 80,
            path: String::from(""),
            protocol: String::from(""),
        };

        if url.starts_with("http://") {
            u.protocol = String::from("http://");
        //   url=  String::from( url.strip_prefix("http://").unwrap_or(&url ));
        } else if url.starts_with("https://") {
            u.protocol = String::from("https://")
        }

        match url.split_once('/') {
            Some((key, value)) => {
                u.domain = String::from(key);
                u.path = String::from(value);
            }
            None => {
                u.domain = String::from(url);
            }
        }

        u
    }
}

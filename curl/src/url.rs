
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

        let mut url_n = url.as_str();

        if url_n.starts_with("http://") {
            u.protocol = String::from("http://");
            url_n = url.strip_prefix("http://").unwrap();
        } else if url.starts_with("https://") {
            u.protocol = String::from("https://");
            url_n = url.strip_prefix("http://").unwrap();
        }

        match url_n.split_once('/') {
            Some((key, value)) => {
                url_n = key;
                u.path = String::from(value);
            }
            None => {}
        }

        match url_n.split_once(':') {
            Some((key, value)) => {
                u.domain = String::from(key);
                u.port = value.parse().expect("not number");
            }
            None => {
                u.domain = String::from(url_n);
            }
        }

        u
    }
}

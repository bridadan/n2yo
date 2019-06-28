struct N2yo {
    api_key: String,
    base_url: String,
}

const DEFAULT_BASE_URL : &str = "https://www.n2yo.com/rest/v1";

impl N2yo {
    fn new(api_key: &str) -> N2yo {
        N2yo {
            api_key: String::from(api_key),
            base_url: String::from(DEFAULT_BASE_URL),
        }
    }

    fn new_with_base_url(api_key: &str, base_url: &str) -> N2yo {
        N2yo {
            api_key: String::from(api_key),
            base_url: String::from(base_url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_constructor() {
        const KEY : &str = "dummy_key";
        let client : N2yo = N2yo::new(KEY);
        assert_eq!(client.api_key, KEY);
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }
    
    #[test]
    fn base_url_constructor() {
        const KEY : &str = "dummy_key";
        const URL : &str = "dummy_url";
        let client : N2yo = N2yo::new_with_base_url(KEY, URL);
        assert_eq!(client.api_key, KEY);
        assert_eq!(client.base_url, URL);
    }
}

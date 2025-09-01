const BANNER: &str = "oat!";

pub struct Engine {}

impl Engine {
    pub fn banner(&self) -> &str {
        BANNER
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_works() {
        let engine = Engine {};
        let result = engine.banner();
        assert_eq!(result, BANNER);
    }
}

pub trait TitleCase {
    fn to_title_case(&self) -> String;
}

impl TitleCase for str {
    fn to_title_case(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl TitleCase for String {
    fn to_title_case(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_title_case() {
        assert_eq!("hello world".to_title_case(), "Hello world".to_string())
    }
}

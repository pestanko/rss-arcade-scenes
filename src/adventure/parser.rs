use slog::{debug, o};

pub struct ScenarioParser {
    log: slog::Logger,
}

impl ScenarioParser {
    pub fn new(log: &slog::Logger) -> Self {
        Self {
            log: log.new(o!("l_name" => "scenario_parser")),
        }
    }

    pub fn parse_token(&self, line: &str) -> Token {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Token::Empty;
        }

        if line.starts_with("---") {
            debug!(self.log, "Parser: found delimiter");
            return Token::Delim;
        }

        if line.starts_with("quit") {
            debug!(self.log, "Parser: found quit token");
            return Token::Quit;
        }

        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() != 2 {
            debug!(self.log, "Parser: invalid token"; "line" => line);
            return Token::Invalid(line.into());
        }
        let (name, value) = (parts[0], parts[1]);
        debug!(self.log, "Parser: found token"; "name" => name, "value" => value);
        match name.trim().to_lowercase().as_str() {
            "name" => Token::Name(value.trim().to_lowercase()),
            "desc" => Token::Desc(value.trim().into()),
            "option" => Token::Option(value.trim().into()),
            _ => Token::Unknown(name.into()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Name(String),
    Desc(String),
    Option(String),
    Quit,
    Delim,
    Empty,
    Invalid(String),
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::Token;
    use slog::o;

    fn get_instance() -> super::ScenarioParser {
        let log = slog::Logger::root(slog::Discard, o!());
        super::ScenarioParser::new(&log)
    }

    #[test]
    fn parse_empty_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token(""), Token::Empty);
        assert_eq!(instance.parse_token("   "), Token::Empty);
        assert_eq!(instance.parse_token("\t  \t"), Token::Empty);
    }

    #[test]
    fn parse_comment_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("#"), Token::Empty);
        assert_eq!(instance.parse_token("#ahoj ssvet"), Token::Empty);
        assert_eq!(instance.parse_token("  ## svet"), Token::Empty);
    }

    #[test]
    fn parse_separator_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("---"), Token::Delim);
        assert_eq!(instance.parse_token("----"), Token::Delim);
        assert_eq!(instance.parse_token("-------"), Token::Delim);
        assert_ne!(instance.parse_token("--"), Token::Delim);
    }

    #[test]
    fn parse_quit_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("quit"), Token::Quit);
        assert_eq!(instance.parse_token("quit:"), Token::Quit);
        assert_eq!(instance.parse_token("quit: yes"), Token::Quit);
        assert_eq!(instance.parse_token("  quit"), Token::Quit);
    }

    #[test]
    fn parse_name_properties_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("name: struct"), Token::Name(String::from("struct")));
        assert_eq!(instance.parse_token("name:"), Token::Name(String::from("")));
        assert_eq!(instance.parse_token("name:      yes"), Token::Name(String::from("yes")));
        assert_eq!(instance.parse_token("name: Yes"), Token::Name(String::from("yes")));
    }

    #[test]
    fn parse_desc_properties_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("desc: Hello world"), Token::Desc(String::from("Hello world")));
        assert_eq!(instance.parse_token("desc:"), Token::Desc(String::from("")));
        assert_eq!(instance.parse_token("desc:      yes"), Token::Desc(String::from("yes")));
        assert_eq!(instance.parse_token("desc: Yes"), Token::Desc(String::from("Yes")));
    }

    #[test]
    fn parse_option_properties_line() {
        let instance = get_instance();
        assert_eq!(instance.parse_token("option: Hello world"), Token::Option(String::from("Hello world")));
        assert_eq!(instance.parse_token("option:"), Token::Option(String::from("")));
        assert_eq!(instance.parse_token("option:      yes"), Token::Option(String::from("yes")));
        assert_eq!(instance.parse_token("option: Yes"), Token::Option(String::from("Yes")));
    }
}

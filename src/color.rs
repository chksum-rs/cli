use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Auto => "auto",
            Self::Never => "never",
        }
    }
}

impl ValueEnum for Color {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        match &input[..] {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err("unrecognized option".to_string()),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Always, Self::Auto, Self::Never]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let possible_value = PossibleValue::new(self.as_str());
        Some(possible_value)
    }
}

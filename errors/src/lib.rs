use from_str_macro::FromStr;

#[derive(Eq, PartialEq, FromStr, Clone, Copy)]
pub enum PostsErrors {
    Unknown,
}


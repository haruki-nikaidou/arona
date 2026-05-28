use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeRange<'a>(pub &'a [ScopeRangeSection<'a>]);

impl ScopeRange<'_> {
    pub fn matches(&self, scope: Scope) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ScopeRangeParseError {
    #[error("Expect a non-empty string at {0}, get an empty string")]
    Empty(usize),
}

impl FromStr for ScopeRange<'_> {
    type Err = ScopeRangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeRangeSection<'a> {
    Lit(&'a str),

    /// `*` matches any one scope
    AnyOne,

    /// `**` matches any number of scopes, including zero
    Wildcard,

    /// `+` matches one or more scopes
    OneOrMore,
}

pub const SYSTEM_SCOPE_RANGE: ScopeRange<'static> =
    ScopeRange(&[ScopeRangeSection::Lit("@isla"), ScopeRangeSection::Wildcard]);

#[derive(Debug, Clone, Copy)]
pub struct Scope<'a>(pub &'a [&'a str]);

impl Scope<'_> {
    pub fn is_system_scope(self) -> bool {
        SYSTEM_SCOPE_RANGE.matches(self)
    }
}

pub const OBJECT_STORAGE_SCOPE: Scope<'static> = Scope(&["@isla", "object_storage"]);

use compact_str::CompactString;
use smallvec::{SmallVec, smallvec};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeRange(pub SmallVec<[ScopeRangeSection; 3]>);

impl ScopeRange {
    pub fn matches(&self, scope: Scope) -> bool {
        fn helper(pattern: &[ScopeRangeSection], scope: &[&str]) -> bool {
            match pattern {
                [] => scope.is_empty(),
                [ScopeRangeSection::Wildcard, rest @ ..] => {
                    // `**` matches zero or more segments
                    (0..=scope.len()).any(|i| helper(rest, &scope[i..]))
                }
                [ScopeRangeSection::OneOrMore, rest @ ..] => {
                    // `+` matches one or more segments
                    (1..=scope.len()).any(|i| helper(rest, &scope[i..]))
                }
                _ if scope.is_empty() => false,
                [ScopeRangeSection::AnyOne, rest_p @ ..] => helper(rest_p, &scope[1..]),
                [ScopeRangeSection::Lit(l), rest_p @ ..] => {
                    l == scope[0] && helper(rest_p, &scope[1..])
                }
            }
        }
        helper(&self.0, scope.0)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ScopeParseError {
    #[error("Expect a non-empty string at {0}, get an empty string")]
    Empty(usize),
}

impl FromStr for ScopeRange {
    type Err = ScopeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: SmallVec<[ScopeRangeSection; 3]> = s
            .split('.')
            .enumerate()
            .map(|(i, part)| {
                if part.is_empty() {
                    return Err(ScopeParseError::Empty(i));
                }
                Ok(match part {
                    "**" => ScopeRangeSection::Wildcard,
                    "*" => ScopeRangeSection::AnyOne,
                    "+" => ScopeRangeSection::OneOrMore,
                    lit => ScopeRangeSection::Lit(CompactString::from(lit)),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(ScopeRange(sections))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeRangeSection {
    Lit(CompactString),

    /// `*` matches any one scope
    AnyOne,

    /// `**` matches any number of scopes, including zero
    Wildcard,

    /// `+` matches one or more scopes
    OneOrMore,
}

pub static SYSTEM_SCOPE_RANGE: LazyLock<ScopeRange> = LazyLock::new(|| {
    ScopeRange(smallvec![
        ScopeRangeSection::Lit(CompactString::const_new("@isla")),
        ScopeRangeSection::Wildcard,
    ])
});

/// A scope is a hard-coded static slice of string segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scope(pub &'static [&'static str]);

impl Scope {
    pub fn is_system_scope(self) -> bool {
        SYSTEM_SCOPE_RANGE.matches(self)
    }
}

pub static OBJECT_STORAGE_SCOPE: Scope = Scope(&["@isla", "memory_repository", "object_storage"]);

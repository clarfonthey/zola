/// Aliases for pages and sections
use core::fmt;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Deserialize)]
#[repr(u16)]
#[serde(try_from = "u32")]
pub enum AliasKind {
    /// Opaque alias; render copy of page.
    Opaque = 200,

    /// Permanent alias.
    Permanent = 301,

    /// Temporary alias; default.
    #[default]
    Temporary = 302,
}
impl TryFrom<u32> for AliasKind {
    type Error = InvalidAlias;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(AliasKind::Opaque),
            301 => Ok(AliasKind::Permanent),
            302 => Ok(AliasKind::Temporary),
            307 => Ok(AliasKind::Temporary),
            _ => Err(InvalidAlias(value)),
        }
    }
}

pub struct InvalidAlias(u32);
impl fmt::Display for InvalidAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid code {} for alias: expected 200, 301, or 302", self.0)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AliasDe {
    Plain(String),
    Advanced {
        path: String,
        #[serde(default = "AliasKind::default")]
        code: AliasKind,
    },
}
impl From<AliasDe> for Alias {
    fn from(value: AliasDe) -> Self {
        match value {
            AliasDe::Plain(path) => Alias { path, code: AliasKind::default() },
            AliasDe::Advanced { path, code } => Alias { path, code },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "AliasDe")]
pub struct Alias {
    /// Slug to alias.
    pub path: String,

    /// Status code for alias.
    pub code: AliasKind,
}
impl From<String> for Alias {
    fn from(path: String) -> Self {
        Alias { path, code: AliasKind::default() }
    }
}
impl From<&str> for Alias {
    fn from(path: &str) -> Self {
        Alias { path: path.to_owned(), code: AliasKind::default() }
    }
}

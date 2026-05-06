//! Defines general resource pack-related data structures.

use std::fmt::Display;

/// The possible kinds of assets resource packs can replace.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetKind {
    Font,
    Image,
    Music,
    Sound,
    Translation(Option<Language>),
}

impl Display for AssetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Font => "Fonts",
                Self::Image => "Images",
                Self::Music => "Music",
                Self::Sound => "Sounds",
                Self::Translation(l) =>
                    if let Some(l) = l {
                        return write!(f, "{l} Text");
                    } else {
                        "Localization"
                    },
            },
        )
    }
}

/// The languages supported by Terrraria.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    English,
    German,
    Italian,
    French,
    Spanish,
    Russian,
    ChineseSimplified,
    ChineseTraditional,
    Portuguese,
    Polish,
    Japanese,
    Korean,
}

impl Language {
    /// Converts a language code to its equivalent [`Language`], or [`None`] if
    /// `code` is not a valid language code.
    pub fn from_lang_code(code: &str) -> Option<Self> {
        match code {
            "en-US" => Some(Self::English),
            "de-DE" => Some(Self::German),
            "it-IT" => Some(Self::Italian),
            "fr-FR" => Some(Self::French),
            "es-ES" => Some(Self::Spanish),
            "ru-RU" => Some(Self::Russian),
            "zh-Hans" => Some(Self::ChineseSimplified),
            "zh-Hant" => Some(Self::ChineseTraditional),
            "pt-BR" => Some(Self::Portuguese),
            "pl-PL" => Some(Self::Polish),
            "ja-JP" => Some(Self::Japanese),
            "ko-KR" => Some(Self::Korean),
            _ => None,
        }
    }

    /// Similar to [`from_lang_code`], but matches with the start of a string
    /// rather than its entirety.
    pub fn from_start_of_string(string: &str) -> Option<Self> {
        const LEN: usize = "en-US".len();
        const LEN_HAN: usize = "zh-Hans".len();

        if string.len() < LEN {
            None
        } else if let Some(l) = Self::from_lang_code(&string[..LEN]) {
            Some(l)
        } else if string.len() < LEN_HAN {
            None
        } else {
            Self::from_lang_code(&string[..LEN_HAN])
        }
    }

    /// Converts this [`Language`] to its equivalent language code.
    pub const fn to_lang_code(self) -> &'static str {
        match self {
            Self::English => "en-US",
            Self::German => "de-DE",
            Self::Italian => "it-IT",
            Self::French => "fr-FR",
            Self::Spanish => "es-ES",
            Self::Russian => "ru-RU",
            Self::ChineseSimplified => "zh-Hans",
            Self::ChineseTraditional => "zh-Hant",
            Self::Portuguese => "pt-BR",
            Self::Polish => "pl-PL",
            Self::Japanese => "ja-JP",
            Self::Korean => "ko-KR",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::English => "English",
                Self::German => "German",
                Self::Italian => "Italian",
                Self::French => "French",
                Self::Spanish => "Spanish",
                Self::Russian => "Russian",
                Self::ChineseSimplified => "Simpl. Chinese",
                Self::ChineseTraditional => "Trad. Chinese",
                Self::Portuguese => "Portuguese",
                Self::Polish => "Polish",
                Self::Japanese => "Japanese",
                Self::Korean => "Korean",
            },
        )
    }
}

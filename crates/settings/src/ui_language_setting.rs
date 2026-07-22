use crate::{self as settings, settings_content::UiLanguageContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, Settings};

/// Display language of the Zed user interface (runtime setting).
/// Zed 用户界面的显示语言(运行时设置)。
///
/// Default: Chinese (zh-CN)
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default, RegisterSetting,
)]
pub enum UiLanguage {
    /// 简体中文(Simplified Chinese)
    #[default]
    Chinese,
    /// English
    English,
}

impl UiLanguage {
    /// 对应的 rust-i18n locale 标识。
    /// The corresponding rust-i18n locale identifier.
    pub fn locale_str(self) -> &'static str {
        match self {
            UiLanguage::Chinese => "zh-CN",
            UiLanguage::English => "en",
        }
    }

    /// 从 locale 标识解析;未知值回退到默认语言(zh-CN)。
    /// Parse from a locale identifier; unknown values fall back to the
    /// default language (zh-CN).
    pub fn from_locale_str(locale: &str) -> Self {
        match locale {
            "en" => UiLanguage::English,
            _ => UiLanguage::Chinese,
        }
    }
}

impl From<UiLanguageContent> for UiLanguage {
    fn from(value: UiLanguageContent) -> Self {
        match value {
            UiLanguageContent::Chinese => Self::Chinese,
            UiLanguageContent::English => Self::English,
        }
    }
}

impl From<UiLanguage> for UiLanguageContent {
    fn from(value: UiLanguage) -> Self {
        match value {
            UiLanguage::Chinese => Self::Chinese,
            UiLanguage::English => Self::English,
        }
    }
}

impl Settings for UiLanguage {
    fn from_settings(s: &crate::settings_content::SettingsContent) -> Self {
        s.ui_language.unwrap().into()
    }
}

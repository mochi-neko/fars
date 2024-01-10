//! Defines the language codes used by the API as locale.

/// The BCP 47 language code.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum LanguageCode {
    /// Arabic (Saudi Arabia)
    ArSA,
    /// Bangla (Bangladesh)
    BnBD,
    /// Bangla (India)
    BnIN,
    /// Czech (Czech Republic)
    CsCZ,
    /// Danish (Denmark)
    DaDK,
    /// Austrian German
    DeAT,
    /// Swiss German
    DeCH,
    /// Standard German (Germany)
    DeDE,
    /// Modern Greek
    ElGR,
    /// Australian English
    EnAU,
    /// Canadian English
    EnCA,
    /// British English
    EnGB,
    /// Irish English
    EnIE,
    /// Indian English
    EnIN,
    /// New Zealand English
    EnNZ,
    /// US English
    EnUS,
    /// English (South Africa)
    EnZA,
    /// Argentine Spanish
    EsAR,
    /// Chilean Spanish
    EsCL,
    /// Colombian Spanish
    EsCO,
    /// Castilian Spanish (Central-Northern Spain)
    EsES,
    /// Mexican Spanish
    EsMX,
    /// American Spanish
    EsUS,
    /// Finnish (Finland)
    FiFI,
    /// Belgian French
    FrBE,
    /// Canadian French
    FrCA,
    /// Swiss French
    FrCH,
    /// Standard French (France)
    FrFR,
    /// Hebrew (Israel)
    HeIL,
    /// Hindi (India)
    HiIN,
    /// Hungarian (Hungary)
    HuHU,
    /// Indonesian (Indonesia)
    IdID,
    /// Swiss Italian
    ItCH,
    /// Standard Italian (Italy)
    ItIT,
    /// Japanese (Japan)
    JaJP,
    /// Korean (Republic of Korea)
    KoKR,
    /// Belgian Dutch
    NlBE,
    /// Standard Dutch (The Netherlands)
    NlNL,
    /// Norwegian (Norway)
    NoNO,
    /// Polish (Poland)
    PlPL,
    /// Brazilian Portuguese
    PtBR,
    /// European Portuguese (Portugal)
    PtPT,
    /// Romanian (Romania)
    RoRO,
    /// Russian (Russian Federation)
    RuRU,
    /// Slovak (Slovakia)
    SkSK,
    /// Swedish (Sweden)
    SvSE,
    /// Indian Tamil
    TaIN,
    /// Sri Lankan Tamil
    TaLK,
    /// Thai (Thailand)
    ThTH,
    /// Turkish (Turkey)
    TrTR,
    /// Chinese (Mainland China, Simplified Characters)
    ZhCN,
    /// Chinese (Hong Kong, Traditional Characters)
    ZhHK,
    /// Chinese (Taiwan, Traditional Characters)
    ZhTW,
}

impl LanguageCode {
    /// Formats the language code as a string.
    pub(crate) fn format(self) -> &'static str {
        match self {
            | LanguageCode::ArSA => "ar-SA",
            | LanguageCode::BnBD => "bn-BD",
            | LanguageCode::BnIN => "bn-IN",
            | LanguageCode::CsCZ => "cs-CZ",
            | LanguageCode::DaDK => "da-DK",
            | LanguageCode::DeAT => "de-AT",
            | LanguageCode::DeCH => "de-CH",
            | LanguageCode::DeDE => "de-DE",
            | LanguageCode::ElGR => "el-GR",
            | LanguageCode::EnAU => "en-AU",
            | LanguageCode::EnCA => "en-CA",
            | LanguageCode::EnGB => "en-GB",
            | LanguageCode::EnIE => "en-IE",
            | LanguageCode::EnIN => "en-IN",
            | LanguageCode::EnNZ => "en-NZ",
            | LanguageCode::EnUS => "en-US",
            | LanguageCode::EnZA => "en-ZA",
            | LanguageCode::EsAR => "es-AR",
            | LanguageCode::EsCL => "es-CL",
            | LanguageCode::EsCO => "es-CO",
            | LanguageCode::EsES => "es-ES",
            | LanguageCode::EsMX => "es-MX",
            | LanguageCode::EsUS => "es-US",
            | LanguageCode::FiFI => "fi-FI",
            | LanguageCode::FrBE => "fr-BE",
            | LanguageCode::FrCA => "fr-CA",
            | LanguageCode::FrCH => "fr-CH",
            | LanguageCode::FrFR => "fr-FR",
            | LanguageCode::HeIL => "he-IL",
            | LanguageCode::HiIN => "hi-IN",
            | LanguageCode::HuHU => "hu-HU",
            | LanguageCode::IdID => "id-ID",
            | LanguageCode::ItCH => "it-CH",
            | LanguageCode::ItIT => "it-IT",
            | LanguageCode::JaJP => "ja-JP",
            | LanguageCode::KoKR => "ko-KR",
            | LanguageCode::NlBE => "nl-BE",
            | LanguageCode::NlNL => "nl-NL",
            | LanguageCode::NoNO => "no-NO",
            | LanguageCode::PlPL => "pl-PL",
            | LanguageCode::PtBR => "pt-BR",
            | LanguageCode::PtPT => "pt-PT",
            | LanguageCode::RoRO => "ro-RO",
            | LanguageCode::RuRU => "ru-RU",
            | LanguageCode::SkSK => "sk-SK",
            | LanguageCode::SvSE => "sv-SE",
            | LanguageCode::TaIN => "ta-IN",
            | LanguageCode::TaLK => "ta-LK",
            | LanguageCode::ThTH => "th-TH",
            | LanguageCode::TrTR => "tr-TR",
            | LanguageCode::ZhCN => "zh-CN",
            | LanguageCode::ZhHK => "zh-HK",
            | LanguageCode::ZhTW => "zh-TW",
        }
    }
}

use std::str::FromStr;
use strum_macros::AsRefStr;
use url::Url;

#[allow(clippy::upper_case_acronyms)]
#[derive(AsRefStr)]
pub enum Token {
    FSH,
    TON,
    UAHT,
    AAVE,
    HCK,
    TRX,
    CRO,
    VQR,
    SHIB,
    TLR,
    LINK,
    MATIC,
    UNI,
    USDC,
    BAT,
    USDT,
    XMR,
    DASH,
    KUB,
    WAVES,
    ADA,
    ETH,
    DOGE,
    KRB,
    BTC,
    LTC,
}

impl Token {
    pub fn try_explorer_url(token: &str) -> Result<Url, TokenError> {
        match token.parse::<Token>() {
            Ok(token) => match token {
                Token::FSH => Ok("https://cardanoscan.io/transaction/".parse().unwrap()),
                Token::TON => Ok("https://tonscan.org/tx/".parse().unwrap()),
                Token::UAHT => Ok("https://polygonscan.com/tx/".parse().unwrap()),
                Token::AAVE => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::HCK => Ok("https://cardanoscan.io/transaction/".parse().unwrap()),
                Token::TRX => Ok("https://tronscan.org/#/transaction/".parse().unwrap()),
                Token::CRO => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::VQR => Ok("https://vqr.vovanchik.net/block.php?hash=".parse().unwrap()),
                Token::SHIB => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::TLR => Ok("https://explorer.talercoin.org/tx/".parse().unwrap()),
                Token::LINK => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::MATIC => Ok("https://polygonscan.com/tx/".parse().unwrap()),
                Token::UNI => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::USDC => Ok("https://polygonscan.com/tx/".parse().unwrap()),
                Token::BAT => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::USDT => Ok("https://tronscan.org/#/transaction/".parse().unwrap()),
                Token::XMR => Ok("https://moneroexplorer.org/tx/".parse().unwrap()),
                Token::DASH => Ok("https://dashblockexplorer.com/tx/".parse().unwrap()),
                Token::KUB => Ok("https://wavesexplorer.com/transactions/".parse().unwrap()),
                Token::WAVES => Ok("https://wavesexplorer.com/transactions/".parse().unwrap()),
                Token::ADA => Ok("https://cardanoscan.io/transaction/".parse().unwrap()),
                Token::ETH => Ok("https://etherscan.io/tx/".parse().unwrap()),
                Token::DOGE => Ok("https://dogechain.info/tx/".parse().unwrap()),
                Token::KRB => Ok("https://explorer.karbo.org/?hash=".parse().unwrap()),
                Token::BTC => Ok("https://btcscan.org/tx/".parse().unwrap()),
                Token::LTC => Ok("https://litecoinblockexplorer.net/tx/".parse().unwrap()),
            },
            Err(e) => Err(e),
        }
    }
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FSH" => Ok(Token::FSH),
            "TON" => Ok(Token::TON),
            "UAHT" => Ok(Token::UAHT),
            "AAVE" => Ok(Token::AAVE),
            "HCK" => Ok(Token::HCK),
            "TRX" => Ok(Token::TRX),
            "CRO" => Ok(Token::CRO),
            "VQR" => Ok(Token::VQR),
            "SHIB" => Ok(Token::SHIB),
            "TLR" => Ok(Token::TLR),
            "LINK" => Ok(Token::LINK),
            "MATIC" => Ok(Token::MATIC),
            "UNI" => Ok(Token::UNI),
            "USDC" => Ok(Token::USDC),
            "BAT" => Ok(Token::BAT),
            "USDT" => Ok(Token::USDT),
            "XMR" => Ok(Token::XMR),
            "DASH" => Ok(Token::DASH),
            "KUB" => Ok(Token::KUB),
            "WAVES" => Ok(Token::WAVES),
            "ADA" => Ok(Token::ADA),
            "ETH" => Ok(Token::ETH),
            "DOGE" => Ok(Token::DOGE),
            "KRB" => Ok(Token::KRB),
            "BTC" => Ok(Token::BTC),
            "LTC" => Ok(Token::LTC),
            token => Err(TokenError::InvalidToken(token.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    InvalidToken(String),
}

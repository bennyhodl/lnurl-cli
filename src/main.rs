use clap::{Parser, Subcommand, Args};
use lnurl::lnurl::LnUrl;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct LnurlArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Encode a url to lnurl.
    Encode(EncodeCommand),
    /// Decode an lnurl string to url.
    Decode(DecodeCommand)
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    /// The url to encode to lnurl.
    pub url: String
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    /// The lnurl encoded string to decode.
    pub lnurl: String
}

fn main() {
    let args: LnurlArgs = LnurlArgs::parse();

    match args.entity_type {
        EntityType::Encode(url) => {
            let lnurl = LnUrl::from_url(url.url).encode();
            println!("{}", lnurl);
        },
        EntityType::Decode(lnurl) => {
            let url = match LnUrl::decode(lnurl.lnurl) {
                Ok(ln) => ln.url,
                Err(_) => "Not a valid lnurl.".to_string()
            };
            println!("{}", url);
        }
    }    
}

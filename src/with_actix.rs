use std::{error::Error, fmt};
use std::pin::Pin;
use futures::future::Future;

use crate::{I18n, Translations, ACCEPT_LANG};

use actix_web::{dev::Payload, FromRequest, HttpRequest, ResponseError};

#[derive(Debug)]
pub struct MissingTranslationsError(String);

impl fmt::Display for MissingTranslationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find translations for {}", self.0)
    }
}

impl Error for MissingTranslationsError {
    fn description(&self) -> &str {
        "Could not find translations"
    }
}

impl ResponseError for MissingTranslationsError {
    // this defaults to an empty InternalServerError response
}

#[derive(Debug)]
pub struct MissingStateError;

impl fmt::Display for MissingStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve state")
    }
}

impl Error for MissingStateError {
    fn description(&self) -> &str {
        "Could not retrieve state"
    }
}

impl ResponseError for MissingStateError {
    // this defaults to an empty InternalServerError response
}

impl FromRequest for I18n {
    type Config = ();
    type Error = actix_web::Error;
    // type Future = Result<Self, Self::Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // let langs = req.app_data::<Translations>().ok_or(MissingStateError)?;
        let langs = req.app_data::<Translations>();

        let lang_pre = req
            .headers()
            .get(ACCEPT_LANG)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("en")
            .split(",")
            .filter_map(|lang| {
                lang
                    // Get the locale, not the country code
                    .split(|c| c == '-' || c == ';')
                    .nth(0)
            });

        Box::pin(async move {
            let langs = langs.ok_or(MissingStateError)?;
            let lang = lang_pre
            // Get the first requested locale we support
            .find(|lang| langs.iter().any(|l| l.0 == &lang.to_string()))
            .unwrap_or("en");

            match langs.iter().find(|l| l.0 == lang) {
                Some(translation) => 
                    Ok(I18n {
                        catalog: translation.1.clone(),
                        lang: translation.0,
                    }),
                None => Err(MissingTranslationsError(lang.to_owned()).into()),
            }
        })
    }
}

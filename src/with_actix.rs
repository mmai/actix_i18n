use std::{error::Error, fmt};
use futures::future::{Ready, ok, err};

use crate::{I18n, Translations, ACCEPT_LANG};

use actix_web::{dev::Payload, FromRequest, HttpRequest, ResponseError};
use actix_web::http::header::HeaderValue;

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
    // type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let langs = req.app_data::<Translations>();
        let langs = match langs {
                Some(langs) => langs.clone(),
                _e => return err(MissingStateError.into())
        };

        let default_lang_header = HeaderValue::from_static("en");
        let lang = req
            .headers()
            .get(ACCEPT_LANG)
            .unwrap_or(&default_lang_header)
            .to_str() 
            .unwrap_or("en") 
            .split(",")
            .filter_map(|lang| {
                lang
                    // Get the locale, not the country code
                    .split(|c| c == '-' || c == ';')
                    .nth(0)
            })
            // Get the first requested locale we support
            .find(|lang| langs.iter().any(|l| l.0 == &lang.to_string()))
            .unwrap_or("en");

        match langs.iter().find(|l| l.0 == lang) {
            Some(translation) => ok(I18n {
                catalog: translation.1.clone(),
                lang: translation.0,
            }),
            None => err(MissingTranslationsError(lang.to_owned()).into()),
        }
    }

}

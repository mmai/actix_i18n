use std::{error::Error, fmt};
//
use std::pin::Pin;
use futures::future::Future;

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
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request<'request>(req: &'request HttpRequest, _: &mut Payload) -> Self::Future {

        // let accept_lang = "en"; 
        let accept_lang = req
            .headers()
            .get(ACCEPT_LANG)
        //     // .await
            .unwrap_or(&HeaderValue::from_static("en"))
            .to_str().ok()
            .unwrap_or("en");
        let langs = req.app_data::<Translations>().ok_or(MissingStateError);
        let langs = match langs {
                Ok(langs) => langs.clone(),
                e => return Box::pin(async {Err(MissingStateError.into())})
        };
        let lang = accept_lang.split(",")
            .filter_map(|lang| {
                lang
                    // Get the locale, not the country code
                    .split(|c| c == '-' || c == ';')
                    .nth(0)
            })
            // Get the first requested locale we support
            .find(|lang| langs.iter().any(|l| l.0 == &lang.to_string()))
                .unwrap_or("en");

        Box::pin( async move {

            match langs.iter().find(|l| l.0 == lang) {
                Some(translation) => Ok(I18n {
                    catalog: translation.1.clone(),
                    lang: translation.0,
                }),
                None => Err(MissingTranslationsError(lang.to_owned()).into()),
            }
        }
        )
    }

}

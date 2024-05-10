use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use color_eyre::eyre::{bail, ContextCompat};
use fluent::{FluentArgs, FluentResource};
use fluent::concurrent::FluentBundle;
use unic_langid::langid;

pub struct LocaleInfo {
    name: &'static str,
    lang: unic_langid::LanguageIdentifier,
    file: &'static str,
}

const LOCALES: &'static [LocaleInfo] = &[
    LocaleInfo {
        name: "en",
        lang: langid!("en-US"),
        file: include_str!("../../resources/en.ftl"),
    },
];

fn get_locales_map() -> &'static HashMap<&'static str, FluentBundle<FluentResource>> {
    static LOCALES_MAP: OnceLock<HashMap<&'static str, FluentBundle<FluentResource>>> = OnceLock::new();
    LOCALES_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for LocaleInfo { name, lang, file } in LOCALES {
            let mut bundle = FluentBundle::new_concurrent(vec![ lang.clone() ]);
            // TODO investigate this
            bundle.set_use_isolating(false);
            let resource = FluentResource::try_new(file.to_string()).unwrap();
            bundle.add_resource(resource).unwrap();
            map.insert(*name, bundle);
        }
        map
    })
}

fn get_message_inner(lang: &str, id: &str, args: Option<&FluentArgs>) -> color_eyre::Result<Cow<'static, str>> {
    let bundle = get_locales_map().get(lang).context("could not get locale for language")?;
    let msg = bundle.get_message(id).with_context(|| format!("`{id}` does not exist in the bundle"))?;
    let val = msg.value().with_context(|| format!("`{id}` in the bundle doesn't have a main value"))?;
    // TODO we should handle the errors
    let mut errors = vec![];
    let msg = bundle.format_pattern(val, args, &mut errors);
    match &*errors {
        [] => {}
        [one] => {
            return Err(one.clone().into())
        }
        [multiple @ ..] => {
            bail!("multiple fluent errors: {multiple:?}")
        }
    }
    Ok(msg)
}

// TODO this really needs to be reworked to look more like Rust's diagnostic translation
#[macro_export]
macro_rules! msg {
    ($lang:expr, $id:literal) => {
        $crate::i18n::get_message($lang, $id)
    };
    ($lang:expr, $id:literal, $($name:ident = $expr:expr),*$(,)?) => {
        $crate::i18n::get_message_with_args($lang, $id, vec![ $( (stringify!($name), ::fluent::types::FluentValue::from($expr)) ),* ].into_iter().collect())
    }
}

pub use msg;

/// Very low level, don't use this!
pub fn get_message(lang: &str, id: &str) -> color_eyre::Result<Cow<'static, str>> {
    get_message_inner(lang, id, None)
}

pub fn get_message_with_args(lang: &str, id: &str, args: FluentArgs) -> color_eyre::Result<Cow<'static, str>> {
    get_message_inner(lang, id, Some(&args))
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_get_message() {
        assert_eq!(
            "No user found. Either the user is not in this server or is unauthenticated.",
            super::get_message("en", "whois_no_user_found").unwrap(),
        );
    }

    #[test]
    fn macro_uses() {
        assert_eq!(
            "No user found. Either the user is not in this server or is unauthenticated.",
            super::msg!("en", "whois_no_user_found").unwrap(),
        );
        assert_eq!(
            "https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/0xDeadbeef",
            super::msg!("en", "user_link", normalized_name = "0xDeadbeef").unwrap(),
        );
    }
}
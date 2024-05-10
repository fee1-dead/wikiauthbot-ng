use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use color_eyre::eyre::{bail, ContextCompat};
use fluent::FluentResource;
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
            let resource = FluentResource::try_new(file.to_string()).unwrap();
            bundle.add_resource(resource).unwrap();
            map.insert(*name, bundle);
        }
        map
    })
}

/// Very low level, don't use this!
pub fn get_message(lang: &str, id: &str) -> color_eyre::Result<Cow<'static, str>> {
    let bundle = get_locales_map().get(lang).context("could not get locale for language")?;
    let msg = bundle.get_message(id).with_context(|| format!("`{id}` does not exist in the bundle"))?;
    let val = msg.value().with_context(|| format!("`{id}` in the bundle doesn't have a main value"))?;
    // TODO we should handle the errors
    let mut errors = vec![];
    let msg = bundle.format_pattern(val, None, &mut errors);
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

#[cfg(test)]
mod tests {
    #[test]
    fn basic_get_message() {
        assert_eq!(
            "no user found. either the user is not in this server or is unauthenticated",
            super::get_message("en", "whois_no_user_found").unwrap(),
        );
    }
}
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use color_eyre::eyre::{ContextCompat, bail};
pub use fluent;
use fluent::concurrent::FluentBundle;
use fluent::{FluentArgs, FluentResource};
use unic_langid::langid;

pub struct LocaleInfo {
    name: &'static str,
    lang: unic_langid::LanguageIdentifier,
    file: &'static str,
}

const LOCALES: &[LocaleInfo] = &[
    LocaleInfo {
        name: "az",
        lang: langid!("az"),
        file: include_str!("../../resources/az.ftl"),
    },
    LocaleInfo {
        name: "be",
        lang: langid!("be"),
        file: include_str!("../../resources/be.ftl"),
    },
    LocaleInfo {
        name: "bn",
        lang: langid!("bn"),
        file: include_str!("../../resources/bn.ftl"),
    },
    LocaleInfo {
        name: "bs",
        lang: langid!("bs"),
        file: include_str!("../../resources/bs.ftl"),
    },
    LocaleInfo {
        name: "da",
        lang: langid!("da"),
        file: include_str!("../../resources/da.ftl"),
    },
    LocaleInfo {
        name: "de",
        lang: langid!("de"),
        file: include_str!("../../resources/de.ftl"),
    },
    LocaleInfo {
        name: "en",
        lang: langid!("en-US"),
        file: include_str!("../../resources/en.ftl"),
    },
    LocaleInfo {
        name: "es",
        lang: langid!("es"),
        file: include_str!("../../resources/es.ftl"),
    },
    LocaleInfo {
        name: "fr",
        lang: langid!("fr"),
        file: include_str!("../../resources/fr.ftl"),
    },
    LocaleInfo {
        name: "it",
        lang: langid!("it"),
        file: include_str!("../../resources/it.ftl"),
    },
    LocaleInfo {
        name: "lo",
        lang: langid!("lo"),
        file: include_str!("../../resources/lo.ftl"),
    },
    LocaleInfo {
        name: "pl",
        lang: langid!("pl"),
        file: include_str!("../../resources/pl.ftl"),
    },
    LocaleInfo {
        name: "ru",
        lang: langid!("ru"),
        file: include_str!("../../resources/ru.ftl"),
    },
    LocaleInfo {
        name: "uk",
        lang: langid!("uk"),
        file: include_str!("../../resources/uk.ftl"),
    },
    LocaleInfo {
        name: "vi",
        lang: langid!("vi"),
        file: include_str!("../../resources/vi.ftl"),
    },
    LocaleInfo {
        name: "zh-hans",
        lang: langid!("zh-hans"),
        file: include_str!("../../resources/zh-hans.ftl"),
    },
];

pub fn get_locales_map() -> &'static HashMap<&'static str, FluentBundle<FluentResource>> {
    static LOCALES_MAP: OnceLock<HashMap<&'static str, FluentBundle<FluentResource>>> =
        OnceLock::new();
    LOCALES_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for LocaleInfo { name, lang, file } in LOCALES {
            let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
            bundle.set_use_isolating(false);
            let resource = FluentResource::try_new(file.to_string()).unwrap();
            bundle.add_resource(resource).unwrap();
            map.insert(*name, bundle);
        }
        map
    })
}

pub fn lang_is_supported(lang: &str) -> bool {
    get_locales_map().contains_key(lang)
}

fn get_message_inner(
    lang: &str,
    id: &str,
    args: Option<&FluentArgs>,
) -> color_eyre::Result<Cow<'static, str>> {
    let bundle = get_locales_map()
        .get(lang)
        .context("could not get locale for language")?;
    let msg = bundle
        .get_message(id)
        .with_context(|| format!("`{id}` does not exist in the bundle"))?;
    let val = msg
        .value()
        .with_context(|| format!("`{id}` in the bundle doesn't have a main value"))?;
    let mut errors = vec![];
    let msg = bundle.format_pattern(val, args, &mut errors);
    match &*errors {
        [] => {}
        [one] => return Err(one.clone().into()),
        multiple => {
            bail!("multiple fluent errors: {multiple:?}")
        }
    }
    Ok(msg)
}

#[macro_export]
macro_rules! msg {
    ($lang:expr, $id:literal) => {
        $crate::i18n::get_message($lang, $id)
    };
    ($lang:expr, $id:literal, $($name:ident = $expr:expr),*$(,)?) => {
        $crate::i18n::get_message_with_args($lang, $id, vec![ $( (stringify!($name), $crate::i18n::fluent::types::FluentValue::from($expr)) ),* ].into_iter().collect())
    }
}

pub use msg;

/// Very low level, don't use this!
pub fn get_message(lang: &str, id: &str) -> color_eyre::Result<Cow<'static, str>> {
    get_message_inner(lang, id, None)
}

pub fn get_message_with_args(
    lang: &str,
    id: &str,
    args: FluentArgs,
) -> color_eyre::Result<Cow<'static, str>> {
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

        assert_eq!(
            "Usuario no encontrado. Puede deberse a que el usuario no esté en el servidor o esté sin autenticar.",
            super::get_message("es", "whois_no_user_found").unwrap(),
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

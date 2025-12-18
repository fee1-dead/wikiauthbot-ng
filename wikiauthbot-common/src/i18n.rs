use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;

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
        name: "bg",
        lang: langid!("bg"),
        file: include_str!("../../resources/bg.ftl"),
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
        name: "ja",
        lang: langid!("ja"),
        file: include_str!("../../resources/ja.ftl"),
    },
    LocaleInfo {
        name: "lo",
        lang: langid!("lo"),
        file: include_str!("../../resources/lo.ftl"),
    },
    LocaleInfo {
        name: "ms",
        lang: langid!("ms"),
        file: include_str!("../../resources/ms.ftl"),
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
        name: "tok",
        lang: langid!("tok"),
        file: include_str!("../../resources/tok.ftl"),
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

static LOCALE_BUNDLES: LazyLock<Vec<FluentBundle<FluentResource>>> = LazyLock::new(|| {
    LOCALES
        .iter()
        .map(|LocaleInfo { name: _, lang, file }| {
            let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
            bundle.set_use_isolating(false);
            let resource = FluentResource::try_new(file.to_string()).unwrap();
            bundle.add_resource(resource).unwrap();
            bundle
        })
        .collect()
});

static NAME_TO_ID: LazyLock<HashMap<&'static str, LanguageId>> = LazyLock::new(|| {
    LOCALES.iter().enumerate().map(|(idx, locale)| (locale.name, LanguageId::new(idx).unwrap())).collect()
});

pub fn all_languages() -> impl Iterator<Item = LanguageId> {
    (0..LOCALES.len()).map(LanguageId::new).map(Option::unwrap)
}

#[derive(Clone, Copy)]
pub struct LanguageId(usize);

impl LanguageId {
    pub fn new(id: usize) -> Option<LanguageId> {
        (id < LOCALES.len()).then_some(LanguageId(id))
    }

    pub fn try_from_str(s: &str) -> Option<LanguageId> {
        NAME_TO_ID.get(s).copied()
    }

    pub fn name(self) -> &'static str {
        LOCALES[self.0].name
    }

    pub fn value(self) -> usize {
        self.0
    }

    fn bundle(self) -> &'static FluentBundle<FluentResource> {
        &LOCALE_BUNDLES[self.0]
    }
}

impl Debug for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

pub struct AtomicLanguageId(AtomicUsize);

impl AtomicLanguageId {
    pub fn new(i: LanguageId) -> AtomicLanguageId {
        AtomicLanguageId(AtomicUsize::new(i.0))
    }
    pub fn load(&self) -> LanguageId {
        LanguageId(self.0.load(Ordering::SeqCst))
    }
    pub fn store(&self, lid: LanguageId) {
        self.0.store(lid.0, Ordering::SeqCst);
    }
}

pub fn lang_is_supported(lang: &str) -> bool {
    LanguageId::try_from_str(lang).is_some()
}

fn get_message_inner(
    lang: LanguageId,
    id: &str,
    args: Option<&FluentArgs>,
) -> color_eyre::Result<Cow<'static, str>> {
    let bundle = lang.bundle();
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
pub fn get_message(lang: LanguageId, id: &str) -> color_eyre::Result<Cow<'static, str>> {
    get_message_inner(lang, id, None)
}

pub fn get_message_with_args(
    lang: LanguageId,
    id: &str,
    args: FluentArgs,
) -> color_eyre::Result<Cow<'static, str>> {
    get_message_inner(lang, id, Some(&args))
}

#[cfg(test)]
mod tests {
    use crate::i18n::LanguageId;

    #[test]
    fn basic_get_message() {
        assert_eq!(
            "No user found. Either the user is not in this server or is unauthenticated.",
            super::get_message(LanguageId::try_from_str("en").unwrap(), "whois_no_user_found").unwrap(),
        );

        assert_eq!(
            "Usuario no encontrado. Puede deberse a que el usuario no esté en el servidor o esté sin autenticar.",
            super::get_message(LanguageId::try_from_str("es").unwrap(), "whois_no_user_found").unwrap(),
        );
    }

    #[test]
    fn macro_uses() {
        assert_eq!(
            "No user found. Either the user is not in this server or is unauthenticated.",
            super::msg!(LanguageId::try_from_str("en").unwrap(), "whois_no_user_found").unwrap(),
        );
        assert_eq!(
            "https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/0xDeadbeef",
            super::msg!(LanguageId::try_from_str("en").unwrap(), "user_link", normalized_name = "0xDeadbeef").unwrap(),
        );
    }
}

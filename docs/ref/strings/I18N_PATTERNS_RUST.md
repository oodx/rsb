# i18n (Internationalization) Patterns in Rust

## Popular i18n Crates & Their Approaches

### 1. **fluent-rs** (Mozilla's Modern i18n System)
Most powerful, used by Firefox. Supports complex pluralization, gender, and formatting.

```rust
// Cargo.toml
[dependencies]
fluent = "0.16"
fluent-bundle = "0.15"
unic-langid = "0.9"

// messages/en-US/main.ftl
welcome = Welcome to { $appName }!
files-found = { $count ->
    [0] No files found
    [one] Found 1 file
   *[other] Found { $count } files
}

// src/i18n.rs
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub struct Localizer {
    bundle: FluentBundle<FluentResource>,
}

impl Localizer {
    pub fn new(locale: &str) -> Self {
        let ftl_string = match locale {
            "es" => include_str!("../messages/es/main.ftl"),
            _ => include_str!("../messages/en-US/main.ftl"),
        };

        let res = FluentResource::try_new(ftl_string.to_string())
            .expect("Failed to parse FTL");

        let lang: LanguageIdentifier = locale.parse()
            .expect("Failed to parse language");

        let mut bundle = FluentBundle::new(vec![lang]);
        bundle.add_resource(res).expect("Failed to add resource");

        Self { bundle }
    }

    pub fn get(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let msg = self.bundle.get_message(key)
            .expect("Message not found");

        let mut errors = vec![];
        let pattern = msg.value().expect("No value");

        self.bundle.format_pattern(&pattern, args, &mut errors)
            .to_string()
    }
}

// Usage
let l10n = Localizer::new("en-US");
println!("{}", l10n.get("welcome", Some(&fluent_args![
    "appName" => "Boxy"
])));
```

### 2. **rust-i18n** (Simple Compile-Time Approach)
Easiest to use, compile-time safety, great for simple apps.

```rust
// Cargo.toml
[dependencies]
rust-i18n = "3"

// locales/en.yml
en:
  hello: "Hello, %{name}!"
  messages:
    errors:
      not_found: "File not found"

// locales/es.yml
es:
  hello: "¡Hola, %{name}!"
  messages:
    errors:
      not_found: "Archivo no encontrado"

// src/main.rs
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    // Auto-detects from LANG env var
    println!("{}", t!("hello", name = "World"));

    // Or set explicitly
    rust_i18n::set_locale("es");
    println!("{}", t!("messages.errors.not_found"));
}
```

### 3. **gettext-rs** (GNU gettext Pattern)
Industry standard, great tooling, used by most Linux apps.

```rust
// Cargo.toml
[dependencies]
gettext-rs = "0.7"
gettext = "0.4"

// src/i18n.rs
use gettext::Catalog;

lazy_static! {
    static ref CATALOG: RwLock<Catalog> = {
        let locale = std::env::var("LANG").unwrap_or("en_US".to_string());
        let path = format!("./locale/{}/LC_MESSAGES/boxy.mo", locale);

        RwLock::new(
            Catalog::parse(
                &std::fs::read(path).unwrap_or_default()
            ).unwrap_or_default()
        )
    };
}

// Macro for convenience
macro_rules! tr {
    ($msg:expr) => {
        CATALOG.read().unwrap().gettext($msg)
    };
    ($msg:expr, $($arg:tt)*) => {
        format!(CATALOG.read().unwrap().gettext($msg), $($arg)*)
    };
}

// Usage
println!("{}", tr!("File not found"));
println!("{}", tr!("Found {} files", count));
```

## 4. **Custom Pattern with Zero-Cost Abstraction**

For maximum performance with i18n support:

```rust
// src/i18n.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Compile all languages into binary
const LANG_EN: &str = include_str!("../lang/en.toml");
const LANG_ES: &str = include_str!("../lang/es.toml");
const LANG_FR: &str = include_str!("../lang/fr.toml");

pub struct I18n {
    strings: HashMap<String, String>,
    fallback: &'static HashMap<String, String>,
}

static FALLBACK: Lazy<HashMap<String, String>> = Lazy::new(|| {
    toml::from_str(LANG_EN).expect("Invalid English strings")
});

impl I18n {
    pub fn new() -> Self {
        let locale = std::env::var("LANG")
            .unwrap_or_else(|_| "en_US".to_string());

        let lang_code = locale.split('_').next().unwrap_or("en");

        let strings = match lang_code {
            "es" => toml::from_str(LANG_ES).unwrap(),
            "fr" => toml::from_str(LANG_FR).unwrap(),
            _ => toml::from_str(LANG_EN).unwrap(),
        };

        Self {
            strings,
            fallback: &*FALLBACK,
        }
    }

    #[inline]
    pub fn get(&self, key: &str) -> &str {
        self.strings.get(key)
            .or_else(|| self.fallback.get(key))
            .map(|s| s.as_str())
            .unwrap_or(key)
    }

    pub fn format(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.get(key).to_string();
        for (placeholder, value) in args {
            result = result.replace(&format!("{{{}}}", placeholder), value);
        }
        result
    }
}

// Global instance
pub static I18N: Lazy<I18n> = Lazy::new(I18n::new);

// Convenience macro
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::I18N.get($key)
    };
    ($key:expr, $($name:ident = $val:expr),+) => {{
        $crate::i18n::I18N.format($key, &[
            $(
                (stringify!($name), &$val.to_string())
            ),+
        ])
    }};
}

// Usage
println!("{}", t!("error.file_not_found"));
println!("{}", t!("welcome.message", name = "User", count = 5));
```

## 5. **Lazy Loading Pattern** (Load only needed language)

```rust
use std::path::PathBuf;

pub struct LazyI18n {
    locale: String,
    cache: RwLock<HashMap<String, String>>,
    base_path: PathBuf,
}

impl LazyI18n {
    pub fn new() -> Self {
        let locale = detect_locale();
        Self {
            locale,
            cache: RwLock::new(HashMap::new()),
            base_path: PathBuf::from("./locales"),
        }
    }

    pub fn get(&self, key: &str) -> String {
        // Check cache first
        if let Some(val) = self.cache.read().unwrap().get(key) {
            return val.clone();
        }

        // Load the file containing this key
        let namespace = key.split('.').next().unwrap_or("common");
        let file_path = self.base_path
            .join(&self.locale)
            .join(format!("{}.yml", namespace));

        if let Ok(content) = std::fs::read_to_string(&file_path) {
            let strings: HashMap<String, String> = serde_yaml::from_str(&content)
                .unwrap_or_default();

            // Cache all strings from this file
            let mut cache = self.cache.write().unwrap();
            cache.extend(strings.clone());

            strings.get(key).cloned().unwrap_or_else(|| key.to_string())
        } else {
            // Fallback to English
            self.get_fallback(key)
        }
    }
}
```

## 6. **Pluralization & Complex Rules**

```rust
// Using fluent for complex pluralization
// messages/en-US/main.ftl
items-in-cart = { $count ->
    [0] Your cart is empty
    [one] You have one item in your cart
   *[other] You have { $count } items in your cart
}

time-remaining = { $hours ->
    [0] { $minutes ->
        [0] Time's up!
        [one] One minute remaining
       *[other] { $minutes } minutes remaining
    }
    [one] { $minutes ->
        [0] One hour remaining
       *[other] One hour and { $minutes } minutes remaining
    }
   *[other] { $hours } hours and { $minutes } minutes remaining
}

// For simple pluralization without fluent
impl I18n {
    pub fn plural(&self, key: &str, count: usize) -> String {
        let plural_key = match count {
            0 => format!("{}.zero", key),
            1 => format!("{}.one", key),
            _ => format!("{}.other", key),
        };

        self.get(&plural_key)
            .replace("{count}", &count.to_string())
    }
}
```

## 7. **Date, Time, and Number Formatting**

```rust
use chrono::{DateTime, Utc};
use num_format::{Locale, ToFormattedString};

pub struct L10nFormatter {
    locale: String,
}

impl L10nFormatter {
    pub fn format_date(&self, date: DateTime<Utc>) -> String {
        match self.locale.as_str() {
            "en_US" => date.format("%m/%d/%Y").to_string(),
            "en_GB" => date.format("%d/%m/%Y").to_string(),
            "de_DE" => date.format("%d.%m.%Y").to_string(),
            "ja_JP" => date.format("%Y年%m月%d日").to_string(),
            _ => date.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn format_number(&self, num: i64) -> String {
        let locale = match self.locale.as_str() {
            "en_US" => Locale::en,
            "de_DE" => Locale::de,
            "fr_FR" => Locale::fr,
            _ => Locale::en,
        };

        num.to_formatted_string(&locale)
    }

    pub fn format_currency(&self, amount: f64) -> String {
        match self.locale.as_str() {
            "en_US" => format!("${:.2}", amount),
            "en_GB" => format!("£{:.2}", amount),
            "de_DE" | "fr_FR" => format!("{:.2} €", amount),
            "ja_JP" => format!("¥{:.0}", amount),
            _ => format!("{:.2}", amount),
        }
    }
}
```

## 8. **Build-Time Optimization Pattern**

```rust
// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Only include requested languages
    let languages = env::var("BOXY_LANGUAGES")
        .unwrap_or_else(|_| "en".to_string());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("i18n_data.rs");

    let mut content = String::new();

    for lang in languages.split(',') {
        let lang_file = format!("locales/{}.yml", lang);
        if Path::new(&lang_file).exists() {
            content.push_str(&format!(
                "const LANG_{}: &str = include_str!(\"{}\");\n",
                lang.to_uppercase(),
                lang_file
            ));
        }
    }

    fs::write(&dest_path, content).unwrap();
}

// src/i18n.rs
include!(concat!(env!("OUT_DIR"), "/i18n_data.rs"));
```

## 9. **Testing i18n Code**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_keys_present_in_all_languages() {
        let languages = ["en", "es", "fr", "de"];
        let mut all_keys = HashSet::new();

        // Collect all keys from all languages
        for lang in &languages {
            let strings = load_language(lang);
            all_keys.extend(strings.keys().cloned());
        }

        // Check each language has all keys
        for lang in &languages {
            let strings = load_language(lang);
            for key in &all_keys {
                assert!(
                    strings.contains_key(key),
                    "Language {} missing key: {}", lang, key
                );
            }
        }
    }

    #[test]
    fn test_placeholder_consistency() {
        let en = load_language("en");
        let es = load_language("es");

        for (key, en_val) in &en {
            if let Some(es_val) = es.get(key) {
                let en_placeholders = extract_placeholders(en_val);
                let es_placeholders = extract_placeholders(es_val);

                assert_eq!(
                    en_placeholders, es_placeholders,
                    "Placeholder mismatch in key: {}", key
                );
            }
        }
    }
}
```

## Performance Comparison

| Method | Init Time | Lookup Time | Memory | Binary Size |
|--------|-----------|-------------|---------|-------------|
| **Compiled-in** | 0ms | 0.3ns | All loaded | +100% strings |
| **rust-i18n** | 0ms | 5ns | All loaded | +100% strings |
| **fluent-rs** | 2ms | 50ns | Per language | +50% per lang |
| **gettext** | 5ms | 20ns | Per language | +30% per lang |
| **Lazy load** | 0ms | 100ns first, 15ns cached | On demand | Minimal |
| **Runtime files** | 2-5ms | 15ns | Per language | Minimal |

## Recommendations for Boxy

### For Simple i18n (2-3 languages, <500 strings):
```rust
// Use rust-i18n or custom compile-time pattern
rust_i18n::i18n!("locales");

fn main() {
    println!("{}", t!("welcome"));
}
```

### For Complex i18n (many languages, plurals, dates):
```rust
// Use fluent-rs
let l10n = Localizer::new(detect_locale());
println!("{}", l10n.get("items-in-cart", Some(&fluent_args![
    "count" => item_count
])));
```

### For Minimal Binary Size:
```rust
// Runtime loading with fallback
static I18N: Lazy<RuntimeI18n> = Lazy::new(|| {
    RuntimeI18n::load_or_embedded()
});
```

## Directory Structure
```
boxy/
├── locales/
│   ├── en/
│   │   ├── common.yml
│   │   ├── errors.yml
│   │   └── help.yml
│   ├── es/
│   │   ├── common.yml
│   │   ├── errors.yml
│   │   └── help.yml
│   └── locale.toml  # metadata
├── src/
│   ├── i18n.rs
│   └── main.rs
└── build.rs  # optional, for build-time processing
```

## Key Takeaways

1. **i18n adds 0-5ms startup cost** - negligible for CLI tools
2. **Lookups are still instant** (5-50ns) after initialization
3. **Choose complexity based on needs** - don't over-engineer
4. **Test all languages** - missing translations are common bugs
5. **Consider fallback chains** - en_US → en → hardcoded
6. **Binary size vs flexibility** - runtime loading for many languages
7. **Use standard formats** - Fluent, gettext, or YAML for tooling support
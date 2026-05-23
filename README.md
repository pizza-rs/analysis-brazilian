<div align="center">

# 🇧🇷 pizza-analysis-brazilian

**Brazilian Portuguese analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--brazilian-blue)](https://github.com/pizza-rs/analysis-brazilian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Brazilian Portuguese analysis with a dedicated stemmer tailored for Brazilian Portuguese
morphology. Unlike the generic Portuguese stemmer, this handles Brazilian-specific verb
conjugations and noun forms.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `brazilian_stem` | Brazilian Portuguese stemmer |
| TokenFilter | `brazilian_stop` | Brazilian Portuguese stop words (203 entries) |
| Analyzer | `brazilian` | Full pipeline: lowercase → stem → stop |

### Brazilian vs Portuguese Stemmer

The Brazilian stemmer handles verb forms and suffixes specific to Brazilian Portuguese
that differ from European Portuguese, providing better recall for BR content.

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_brazilian::register_all(&mut factory);

let analyzer = factory.get_analyzer("brazilian").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-brazilian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["brazilian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>

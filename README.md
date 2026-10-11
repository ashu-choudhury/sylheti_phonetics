# sylheti_phonetics 🗣️

[![Crates.io](https://img.shields.io/crates/v/sylheti_phonetics.svg)](https://crates.io/crates/sylheti_phonetics)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

> **"We don't know if our language will still be spoken 50 years from now. But as long as we are alive today, we will fight to keep its voice heard."**

---

## ❤️ Why This Library Exists: A Letter From a Native Son

I am from South Assam—the green valleys of Cachar, Karimganj, and Hailakandi—where the Barak flows and where our elders spoke Sylheti with every breath. 

Sylheti is an endangered language. Open any linguistic report, and you will see the warnings: our historical *Sylheti Nagri* script was displaced, our grammar is pushed to the margins, and our children are growing up with their mother tongue fading from their tongues. In 50 years, will anyone still speak Sylheti here? Honestly, none of us know. 

**All I know is that we are here right now, in today's digital world, and we have a duty to keep our mother tongue alive.**

The idea for this project was born on an ordinary evening. I was sitting at home, opening WhatsApp to listen to our family messages. As a screen reader user, speech synthesizers are my eyes. They are how I read every text, every prayer, and every message from home.

My family members were texting the way everyone in our valley texts—in Romanized English letters:

> *"Bhai, kila aso? Ghoro boiya aisi, maa-e khani bonaiso, shonge aiba ni?"*

I tapped to play the message, and my English text-to-speech engine completely butchered it. It turned *"kila"* into tequila, mutilated *"khani"* into broken electronic noise, and made the words of my own mother completely unrecognizable. When I tried standard Bengali voices, they failed too—flattening our authentic `[x]` (*kh*), our soft `[f]`, and our rich `[z]` into sounds no Sylheti person ever speaks.

I sat there with a heavy heart. **Our language isn't dying because we stopped loving it; it is being suffocated because the software of our modern world acts like we don't exist.**

You cannot save a mother tongue by lecturing youth or forcing people to abandon how they chat. **You save a language by building tools that work for real life.** People text in Roman letters. People mix English into their daily speech. 

So I wrote **`sylheti_phonetics`**—my very first Rust library, built with every ounce of my heart and care, so that any phone, any screen reader, and any application on Earth can speak our mother tongue naturally.

---

## 💡 The Core Innovation

Instead of demanding massive AI servers or complex models, `sylheti_phonetics` is engineered with clean linguistic precision:

```
[ WhatsApp Chat: "Bhai, ami office-o meeting-e aisi, please call khoro." ]
                                       │
                                       ▼
                     ┌───────────────────────────────────┐
                     │         sylheti_phonetics         │
                     │  - Word Dictionary Lookup         │
                     │  - English Loanword Classifier    │
                     │  - Morphological Suffix Stripper  │
                     │  - Phonetic Nukta Engine          │
                     └───────────────────────────────────┘
                                       │
        ┌──────────────────────────────┼──────────────────────────────┐
        ▼                              ▼                              ▼
 [ Natural TTS Mode ]          [ Strict Latin Mode ]          [ Historical Nagri ]
   (For Vocalizer TTS)            (For Visual Text)           (Unicode U+A800)
"भाइ, आमी ऑफ़िस-ओ              "भाइ, आमी office-ओ            "ꠜꠣꠁ, ꠀꠝꠤ office-ꠅ
 मीटिंग-ए आइसी,                  meeting-ए आइसी,                meeting-ꠄ ꠀꠁꠍꠤ,
 प्लीज़ कॉल ख़ोरो."               please call ख़ोरो."            please call ꠇꠞꠧ."
```

1. **The Devanagari TTS Bridge:** Hindi Devanagari features Nukta characters (`ख़`, `फ़`, `ज़`, `श`) that mirror authentic Sylheti mouth movements **1-to-1**. When converted to this phonetic script, standard Hindi synthesizers (like Nuance Vocalizer's *Lekha* or *Neel*) pronounce Sylheti with **stunning, lifelike authenticity**.
2. **Natural Mode (Default):** Seamlessly integrates loanwords like `office` $\to$ `ऑफ़िस`, `meeting` $\to$ `मीटिंग`, `call` $\to$ `कॉल` so that the TTS voice speaks in **one continuous, natural, human breath** without voice glitches or awkward pauses.
3. **Strict Mode:** Preserves English loanwords in pure Latin characters for visual chat feeds.
4. **Heritage Revival:** Converts text simultaneously into **historical Sylheti Nagri Unicode (`U+A800..U+A82F`)** and standard **Bengali script**.

---

## ⚡ Technical Architecture & Developer Guide

### 📦 Installation

Add `sylheti_phonetics` to your `Cargo.toml`:

```toml
[dependencies]
sylheti_phonetics = "0.1.0"
```

Or install directly via Cargo:

```bash
cargo add sylheti_phonetics
```

---

### 🛠️ Comprehensive Usage Guide

#### 1. Audio-Ready Speech Synthesis (Natural Mode — Recommended)

This is the primary function for screen readers, voice bots, and TTS applications. It converts Romanized Sylheti while phonetically integrating English loanwords to guarantee smooth, uninterrupted audio playback:

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let message = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    
    let speech_text = to_hindi_phonetic(message);
    println!("{}", speech_text);
    // Output: "भाइ, आमी ऑफ़िस-ओ मीटिंग-ए आइसी, प्लीज़ कॉल ख़ोरो."
    // Pass this directly to your Hindi Vocalizer / SAPI / Android TTS!
}
```

#### 2. Strict Latin Mode (For Visual Chat & Screen Readers)

If your application displays chat text visually on screen and needs English words preserved strictly in Latin characters:

```rust
use sylheti_phonetics::to_hindi_phonetic_strict;

fn main() {
    let message = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    
    let visual_text = to_hindi_phonetic_strict(message);
    println!("{}", visual_text);
    // Output: "भाइ, आमी office-ओ meeting-ए आइसी, please call ख़ोरो."
}
```

#### 3. Preserving Our Written Heritage (Sylheti Nagri & Bengali)

Bridge modern casual chat back to our historical roots:

```rust
use sylheti_phonetics::{to_bengali, to_nagri};

fn main() {
    let greeting = "Kila aso bhai? Khani khaiso ni?";

    // Standard Bengali Script
    println!("Bengali: {}", to_bengali(greeting));
    // Output: "কিলা আসো ভাই? খানি খাইসো নি?"

    // Historical Sylheti Nagri Unicode (U+A800..U+A82F)
    println!("Nagri:   {}", to_nagri(greeting));
    // Output: "ꠇꠤꠟꠣ ꠀꠍꠧ ꠜꠣꠁ? ꠈꠣꠘꠤ ꠈꠣꠁꠍꠧ ꠘꠤ?"
}
```

#### 4. Fine-Grained Developer Options

Configure every aspect of the transformation engine:

```rust
use sylheti_phonetics::{convert, ConversionOptions, EnglishMode, TargetScript};

fn main() {
    let options = ConversionOptions {
        target: TargetScript::HindiVocalizer,
        english_mode: EnglishMode::Natural, // or EnglishMode::Strict, EnglishMode::RawPhonetic
    };

    let text = "Mobile-o message check khoriya dekho.";
    let output = convert(text, &options);
    println!("{}", output);
    // Output: "मोबाइल-ओ मैसेज चेक ख़ोरिया देख़ो."
}
```

#### 5. Dictionary Lookup & Metadata

Inspect linguistic metadata for individual words:

```rust
use sylheti_phonetics::find_word;

fn main() {
    if let Some(entry) = find_word("khosoba") {
        println!("Canonical : {}", entry.canonical_roman);  // "khosoba"
        println!("Hindi TTS  : {}", entry.hindi_phonetic);   // "ख़ोशोबा"
        println!("Bengali    : {}", entry.bengali_script);   // "খোশোবা"
        println!("Nagri      : {}", entry.nagri_script);     // "ꠈꠧꠡꠛꠣ"
        println!("Meaning    : {}", entry.english_meaning);  // "fragrant / sweet aroma"
        println!("Part of Sp : {:?}", entry.pos);            // Adjective
    }
}
```

---

## 🌍 Platform Integrations

Because this crate is written in 100% safe, dependency-free Rust, it can be deployed everywhere:

* **NVDA Screen Reader & Windows SAPI:** Compile as a C-ABI dynamic library (`cdylib` `.dll`) to intercept speech calls in screen readers.
* **Android (JNI):** Compile as `.so` or package into an `.aar` library to run natively inside Android accessibility services.
* **Web & Browsers (WASM):** Compile to `wasm32-unknown-unknown` with `wasm-bindgen` for instant, zero-latency browser speech tools.

---

## 🧪 Testing & Verification

Run the comprehensive unit test and doctest suite:

```bash
cargo test
```

Run the live interactive demonstration with real-world examples:

```bash
cargo run --example demo
```

---

## ✊ An Open Call to Everyone

Whether you are from Silchar, Karimganj, Hailakandi, Sylhet, London, or New York—this project belongs to all of us. 

We might not know what will happen to our language in 50 years. But today, right here, right now, we can write code, add words, refine pronunciations, and give our mother tongue the digital dignity it deserves.

Contributions, word additions, and feedback are welcomed with open arms.

---

## 🔬 Citations & Academic References

1. **End-to-end variational speech synthesis for the endangered Sylheti Nagri language** (Md. Ataullha, Soumik Paul Jisun, M. Shahidur Rahman, *Systems and Soft Computing*, 2026, [DOI: 10.1016/j.sasc.2026.200496](https://doi.org/10.1016/j.sasc.2026.200496))
2. **SOAS Sylheti Project** — Descriptive Lexicon & Nagri revival (SOAS University of London)
3. **Vashantor Benchmark Dataset** — Regional Dialect Benchmark (Mukaffi et al.)

---

## 📄 License

Dual-licensed under either:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

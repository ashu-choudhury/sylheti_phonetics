# sylheti_phonetics 🗣️

[![Crates.io](https://img.shields.io/crates/v/sylheti_phonetics.svg)](https://crates.io/crates/sylheti_phonetics)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

> A high-performance Rust phonetic dictionary and transliteration engine. Converts Romanized Sylheti chat text into **Vocalizer-tuned Hindi Devanagari (TTS mode)**, standard **Bengali script**, and historical **Sylheti Nagri (`U+A800..U+A82F`)**, with intelligent bilingual code-switching.

---

## 📌 The Problem & The Origin

I am from South Assam (the Barak Valley region—Cachar, Karimganj, and Hailakandi), where Sylheti is our daily spoken language.

As a screen reader user, I rely entirely on text-to-speech engines to read incoming messages on my phone and computer. One day, I opened WhatsApp to check messages from friends and family. Like most people in our region, everyone types Sylheti using Roman English letters:

> *"Bhai, kila aso? Khani khaiso ni? Ghoro boiya aisi, shonge aiba ni?"*

When my English text-to-speech engine tried to read this, the result was incomprehensible. It butchered nearly every single word—pronouncing *"kila"* like tequila, mutilating *"khani"* into robotic noise, and turning ordinary everyday conversation into total gibberish. I could not understand a single sentence.

Switching to a standard Bengali voice didn't solve the problem either. Standard Bengali phonology lacks Sylheti's distinct fricatives—specifically the velar fricative `[x]` (the rough *kh* in *khani*), the labial `[f]` (in *fani*), and the `[z]` sound.

Sylheti is recognized as an endangered language, and while a small software library isn't going to single-handedly save an endangered tongue from disappearing over the next 50 years, at the very least, our everyday digital tools shouldn't make our language unreadable today.

---

## 💡 How This Library Fixes It

Rather than requiring heavy machine learning models, `sylheti_phonetics` solves this through a practical phonetic bridge:

1. **The Devanagari Phonetic Bridge:**  
   Hindi Devanagari includes Nukta consonants (`ख़`, `फ़`, `ज़`) that map directly to authentic Sylheti sounds. When Romanized Sylheti is converted into this phonetic form, standard Hindi TTS voices (such as Nuance Vocalizer's *Lekha* or *Neel*) pronounce Sylheti naturally and clearly.

2. **Bilingual Chat Handling (Dual Modes):**  
   Everyday messages frequently mix English with Sylheti (*"Bhai, ami office-o meeting-e aisi, please call khoro"*). This library provides two modes:
   * **Natural Mode (Default):** Converts common English loanwords into phonetic Devanagari (`ऑफ़िस-ओ`, `मीटिंग-ए`, `प्लीज़`, `कॉल`) so the TTS voice reads the entire sentence smoothly in a single accent without voice switching or audio stuttering.
   * **Strict Mode:** Keeps English words in their original Latin script (`office-ओ`, `meeting-ए`, `please`, `call`) for applications that need visual script separation.

3. **Multi-Script Output:**  
   Beyond TTS audio generation, the library can also output standard **Bengali script** and historical **Sylheti Nagri script (`U+A800..U+A82F`)**.

---

## 📦 Installation

Add `sylheti_phonetics` to your `Cargo.toml`:

```toml
[dependencies]
sylheti_phonetics = "0.1.0"
```

Or via Cargo CLI:

```bash
cargo add sylheti_phonetics
```

---

## 🚀 Usage Guide

### 1. Generating Audio-Ready Text for TTS (Natural Mode)

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let input = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    let tts_text = to_hindi_phonetic(input);

    println!("{}", tts_text);
    // Output: "भाइ, आमी ऑफ़िस-ओ मीटिंग-ए आइसी, प्लीज़ कॉल ख़ोरो."
    // Pass this output directly to your Hindi Vocalizer or SAPI synthesizer.
}
```

### 2. Preserving Latin English Words (Strict Mode)

```rust
use sylheti_phonetics::to_hindi_phonetic_strict;

fn main() {
    let input = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    let strict_text = to_hindi_phonetic_strict(input);

    println!("{}", strict_text);
    // Output: "भाइ, आमी office-ओ meeting-ए आइसी, please call ख़ोरो."
}
```

### 3. Converting to Bengali and Sylheti Nagri Scripts

```rust
use sylheti_phonetics::{to_bengali, to_nagri};

fn main() {
    let input = "Kila aso bhai? Khani khaiso ni?";

    // Standard Bengali Script
    println!("Bengali: {}", to_bengali(input));
    // Output: "কিলা আসো ভাই? খানি খাইসো নি?"

    // Historical Sylheti Nagri Script (Unicode U+A800..U+A82F)
    println!("Nagri:   {}", to_nagri(input));
    // Output: "ꠇꠤꠟꠣ ꠀꠍꠧ ꠜꠣꠁ? ꠈꠣꠘꠤ ꠈꠣꠁꠍꠧ ꠘꠤ?"
}
```

### 4. Custom Configuration

```rust
use sylheti_phonetics::{convert, ConversionOptions, EnglishMode, TargetScript};

fn main() {
    let options = ConversionOptions {
        target: TargetScript::HindiVocalizer,
        english_mode: EnglishMode::Natural, // or EnglishMode::Strict, EnglishMode::RawPhonetic
    };

    let result = convert("Mobile-o message check khoro.", &options);
    println!("{}", result);
    // Output: "मोबाइल-ओ मैसेज चेक ख़ोरो."
}
```

### 5. Dictionary Inspection

```rust
use sylheti_phonetics::find_word;

fn main() {
    if let Some(entry) = find_word("khani") {
        println!("Canonical: {}", entry.canonical_roman);  // "khani"
        println!("Hindi TTS: {}", entry.hindi_phonetic);   // "ख़ानी"
        println!("Bengali:   {}", entry.bengali_script);   // "খানি"
        println!("Nagri:     {}", entry.nagri_script);     // "ꠈꠣꠘꠤ"
        println!("Meaning:   {}", entry.english_meaning);  // "food / meal"
    }
}
```

---

## ⚡ Technical Highlights

* **Zero Runtime Dependencies:** Pure Rust standard library.
* **Fast & Lightweight:** Processes typical chat strings in microseconds with minimal memory overhead.
* **Cross-Platform:** Designed to compile cleanly as a library for desktop software (NVDA screen reader via C-ABI), mobile (Android via JNI), or web (WebAssembly).

---

## 🧪 Testing & Demo

Run unit and documentation tests:

```bash
cargo test
```

Run the included example demo:

```bash
cargo run --example demo
```

---

## 🔬 Citations & References

1. **End-to-end variational speech synthesis for the endangered Sylheti Nagri language** (Md. Ataullha, Soumik Paul Jisun, M. Shahidur Rahman, *Systems and Soft Computing*, 2026, [DOI: 10.1016/j.sasc.2026.200496](https://doi.org/10.1016/j.sasc.2026.200496))
2. **SOAS Sylheti Project** — Descriptive Lexicon & Nagri documentation (SOAS University of London)
3. **Vashantor Benchmark Dataset** — Regional Dialect Benchmark (Mukaffi et al.)

---

## 📄 License

Dual-licensed under either:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

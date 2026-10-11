# sylheti_phonetics 🗣️

[![Crates.io](https://img.shields.io/crates/v/sylheti_phonetics.svg)](https://crates.io/crates/sylheti_phonetics)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

> **A high-performance Rust phonetic dictionary & transliteration engine for Sylheti.**  
> Built out of love for a mother tongue: bridges Romanized family WhatsApp chat to **Vocalizer-tuned Hindi Devanagari (TTS mode)**, **Bengali script**, and historical **Sylheti Nagri (`U+A800..U+A82F`)**, with native **bilingual English code-switching**.

---

## ❤️ Why the Hell Did I Build This?

One day, I opened WhatsApp to listen to messages from my family and friends.

Like millions of native Sylheti speakers from South Assam (Cachar, Karimganj, Hailakandi), the Surma Valley, and across the globe, my people don't type in Bengali script or historical Nagri font. We type the way we speak, using English letters on our phones:

> *"Bhai, kila aso? Ghoro aisi, maa-e khani bonaiso, shonge aiba ni?"*

As a screen reader and text-to-speech user, I tapped to play the messages. 

**My English TTS engine completely sucked.** It butchered every single word. It tried to pronounce *"kila"* like tequila, *"khani"* like a broken robotic drone, and *"bairono"* into unintelligible gibberish. I couldn't understand a single sentence of my own family's voices.

When I tried switching to a standard Bengali TTS synthesizer, it wasn't much better. Bengali TTS flattens the rich, authentic sounds of Sylheti—it cannot pronounce our velar fricative `[x]` (our *kh* in *khani*), our labial `[f]` (in *fani*), or our distinct `[z]` sounds.

**I realized our mother tongue is slipping through the digital cracks.** Not because our language is dead—tens of millions of us speak it every single day—but because the digital tools we rely on treat us like we don't exist.

I asked myself: *How do we fix this mess?*

We cannot revive a language by lecturing people or forcing them to type in a script they were never taught in school. **The only real way to save an endangered mother tongue is to build solutions that meet people where they actually are.** 

People chat in Romanized letters. People mix English loanwords into their sentences. So I sat down and built **`sylheti_phonetics`**.

This is my first published Rust library, and I am deeply proud of it.

---

## 💡 How It Works

Instead of trying to train an enormous 5-gigabyte neural network from scratch, `sylheti_phonetics` leverages a brilliant linguistic insight:

1. **The Hindi Phonetic Bridge:**
   Hindi Devanagari script features Nuktas (`ख़`, `फ़`, `ज़`, `श`) that map **1-to-1** with authentic Sylheti fricatives. When Romanized Sylheti is accurately mapped to Devanagari, standard Hindi Vocalizer synthesizers (like *Lekha* or *Neel*) read it aloud with an **astonishingly natural, native Sylheti accent**.
2. **Smart English Code-Switching:**
   In real life, people write:
   > *"Bhai, ami office-o meeting-e aisi, please later call khoro."*
   
   `sylheti_phonetics` detects English words (`office`, `meeting`, `please`, `later`, `call`) and **leaves them completely intact as English**, while converting dedicated Sylheti words into Devanagari phonetics. This allows bilingual TTS voices to read the English words with their native English dictionary without missing a beat!
3. **Heritage Preservation:**
   Every word can also be translated directly into historical **Sylheti Nagri script (`U+A800..U+A82F`)** and standard **Bengali script**.

---

## 📦 What's Inside

* **Zero External Dependencies:** Built with pure, safe Rust standard library. Runs in microseconds with near-zero memory footprint.
* **Curated Word Database (`SYLHETI_WORDS`):** High-frequency Sylheti greetings, family relations, verbs, food, and regional Barak Valley expressions.
* **Intelligent Morphological Suffix Parsing:** Understands Sylheti inflections attached to English loanwords (`office-o` $\to$ `office-ओ`, `meeting-e` $\to$ `meeting-ए`).
* **Deterministic Fallback Engine (`rules.rs`):** Converts out-of-vocabulary words deterministically.
* **Deploy Anywhere:** Ready to integrate into:
  * Rust crates (`cargo add sylheti_phonetics`)
  * NVDA screen reader plugins & Windows SAPI (via C-ABI `.dll`)
  * Android accessibility services (via JNI `.aar`)
  * Web applications (via WebAssembly `.wasm`)

---

## 🚀 Quick Start

Add `sylheti_phonetics` to your `Cargo.toml`:

```toml
[dependencies]
sylheti_phonetics = "0.1.0"
```

### 1. Audio-Ready Text for Vocalizer TTS

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let input = "Kila aso bhai? Khani khaiso ni?";
    let tts_text = to_hindi_phonetic(input);
    
    // Send this directly into your Hindi TTS synthesizer!
    println!("{}", tts_text);
    // Output: "कीला आसो भाइ? ख़ानी ख़ाइसो नी?"
}
```

### 2. Family WhatsApp Message (Code-Switching in Action)

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let message = "Bhai, ami office-o meeting-e aisi, please later call khoro.";
    let tts_text = to_hindi_phonetic(message);
    
    println!("{}", tts_text);
    // Output: "भाइ, आमी office-ओ meeting-ए आइसी, please later call ख़ोरो."
    // Notice: "office", "meeting", "please", "later", "call" remain intact English!
}
```

### 3. Preserving Our Written Heritage (Nagri & Bengali)

```rust
use sylheti_phonetics::{to_bengali, to_nagri};

fn main() {
    let input = "Kila aso?";

    println!("Bengali: {}", to_bengali(input)); // "কিলা আসো?"
    println!("Nagri:   {}", to_nagri(input));   // "ꠇꠤꠟꠣ ꠀꠍꠧ?"
}
```

---

## 🧪 Testing & Demo

Run the test suite:

```bash
cargo test
```

Run the interactive demo with real chat examples:

```bash
cargo run --example demo
```

---

## ✊ An Open Invitation

We must protect our mother languages at all costs. Not with anger, but with engineering. 

If you are a Sylheti speaker, a linguist, or a developer, please contribute! Add words from your region (whether Sylhet, Cachar, Karimganj, Hailakandi, or London), refine pronunciations, and help build tools that keep our language thriving in the digital age.

---

## 🔬 Research & Citations

1. **End-to-end variational speech synthesis for the endangered Sylheti Nagri language** (Md. Ataullha, Soumik Paul Jisun, M. Shahidur Rahman, *Systems and Soft Computing*, 2026, [DOI: 10.1016/j.sasc.2026.200496](https://doi.org/10.1016/j.sasc.2026.200496))
2. **SOAS Sylheti Project** — Descriptive Lexicon & Nagri revival (SOAS University of London)
3. **Vashantor Benchmark Dataset** — Regional Dialect Benchmark (Mukaffi et al.)

---

## 📄 License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

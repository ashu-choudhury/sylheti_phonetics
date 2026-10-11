# sylheti_phonetics 🗣️

[![Crates.io](https://img.shields.io/crates/v/sylheti_phonetics.svg)](https://crates.io/crates/sylheti_phonetics)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

> **Born from frustration, built with love for our mother tongue.**  
> A high-performance native Rust library that transforms everyday Romanized Sylheti chat messages into **fluent, authentic speech** through Hindi Vocalizer TTS, standard **Bengali script**, and historical **Sylheti Nagri (`U+A800..U+A82F`)**.

---

## ❤️ Why Did I Build This?

One evening, I opened WhatsApp to listen to messages in our family group.

My family and friends from South Assam (Cachar, Karimganj, Hailakandi) and the Surma Valley were texting about their day, dinner plans, and evening tea. Like millions of Sylheti speakers worldwide, nobody was typing in formal Bengali script or historical Nagri font. We write the way we talk, using English letters on our phones:

> *"Bhai, kila aso? Ghoro boiya aisi, maa-e khani bonaiso, shonge aiba ni?"*

As a screen reader user who relies entirely on text-to-speech to interact with the digital world, I pressed play.

**My English TTS engine sounded pathetic.** It stumbled, robotic and cold, butchering every word. It pronounced *"kila"* like tequila, mutilated *"khani"* into an alien noise, and turned my grandmother's and mother's expressions into painful gibberish. I could not understand a single sentence from the people I love most.

I switched to a standard Bengali voice, hoping it would help. It didn't. Bengali speech engines completely flatten the living soul of Sylheti. They cannot pronounce our velar fricative `[x]` (our deep *kh* in *khani*), our soft labial `[f]` (in *fani*), or our distinct `[z]` sounds.

Sitting there in silence, a heavy realization hit me:  
**Our mother tongue is being silenced by the very software we use every day.**

Not because our language is dead. Millions of us speak it every waking minute with pride. But modern speech technology acts like we don't exist.

I asked myself: *How do we fix this mess?*

You cannot save a language by lecturing teenagers on social media or demanding that our parents learn complex keyboard layouts. **The only true way to revive an endangered mother tongue is to build technology that meets our people exactly where they are.**

People chat in Romanized English letters. People naturally mix English loanwords into their conversations. 

So I wrote **`sylheti_phonetics`**.

This is my very first Rust library, and it represents my heart, my heritage, and my identity.

---

## 💡 How It Solves the Problem

Instead of demanding gigabytes of AI models and expensive cloud servers, this engine works through linguistic brilliance:

### 1. The Devanagari Phonetic Bridge
Hindi Devanagari script features Nukta consonants (`ख़`, `फ़`, `ज़`, `श`) that mirror authentic Sylheti mouth movements **one-to-one**. When Romanized Sylheti is mapped to this phonetic structure, standard Hindi TTS synthesizers (like Nuance Vocalizer's *Lekha* or *Neel*) pronounce Sylheti with **breath-taking, natural authenticity**.

### 2. Full Developer Control: Natural vs. Strict Modes
Real people don't speak 100% textbook Sylheti. They chat:
> *"Bhai, ami office-o meeting-e aisi, please call khoro."*

When a synthesizer encounters sudden English Latin letters in the middle of Indic text, it often hiccups, stutters, or awkwardly shifts voice profiles mid-sentence. `sylheti_phonetics` gives developers full control:

* **Natural Mode (Default):** Intelligently recognizes loanwords like `office`, `meeting`, `please`, `call` and maps them to smooth Indian phonetic equivalents (`ऑफ़िस-ओ`, `मीटिंग-ए`, `प्लीज़`, `कॉल`). The speech synthesizer reads the whole sentence in **one continuous, natural, human breath**.
* **Strict Mode:** Keeps English words strictly in raw Latin characters (`office-ओ`, `meeting-ए`, `please`, `call`) for developers who need distinct visual script boundaries.
* **Raw Phonetic Mode:** Transliterates every character strictly through Sylheti phonetic rules.

### 3. Preserving Our Heritage (Nagri & Bengali)
Every sentence can simultaneously be converted to **historical Sylheti Nagri Unicode (`U+A800..U+A82F`)** and standard **Bengali script**, keeping our ancient literary tradition alive.

---

## ⚡ Performance & Design

* **Zero External Dependencies:** Built 100% on pure, safe Rust standard library.
* **Microsecond Speed:** Processes whole chat threads in microseconds with negligible RAM.
* **Universal Portability:**
  * Direct Rust crate: `cargo add sylheti_phonetics`
  * Screen readers: Compiles to C-ABI (`.dll` / `.so`) for NVDA plugins
  * Mobile: Ready for Android integration via JNI (`.aar`)
  * Web: Compiles directly to WebAssembly (`.wasm`) for in-browser speech apps

---

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
sylheti_phonetics = "0.1.0"
```

### 1. Natural Mode (Recommended for TTS Synthesizers)

```rust
use sylheti_phonetics::to_hindi_phonetic;

fn main() {
    let chat = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    
    // Smooth, uninterrupted speech synthesis:
    let audio_text = to_hindi_phonetic(chat);
    println!("{}", audio_text);
    // Output: "भाइ, आमी ऑफ़िस-ओ मीटिंग-ए आइसी, प्लीज़ कॉल ख़ोरो."
}
```

### 2. Strict Mode (Preserving Latin Characters)

```rust
use sylheti_phonetics::to_hindi_phonetic_strict;

fn main() {
    let chat = "Bhai, ami office-o meeting-e aisi, please call khoro.";
    
    let strict_text = to_hindi_phonetic_strict(chat);
    println!("{}", strict_text);
    // Output: "भाइ, आमी office-ओ meeting-ए आइसी, please call ख़ोरो."
}
```

### 3. Reviving Our Written Script (Nagri & Bengali)

```rust
use sylheti_phonetics::{to_bengali, to_nagri};

fn main() {
    let text = "Kila aso bhai? Khani khaiso ni?";

    println!("Bengali: {}", to_bengali(text));
    // Output: "কিলা আসো ভাই? খানি খাইসো নি?"

    println!("Nagri:   {}", to_nagri(text));
    // Output: "ꠇꠤꠟꠣ ꠀꠍꠧ ꠜꠣꠁ? ꠈꠣꠘꠤ ꠈꠣꠁꠍꠧ ꠘꠤ?"
}
```

---

## 🧪 Testing & Demo

Run the test suite:

```bash
cargo test
```

Run the live interactive demonstration:

```bash
cargo run --example demo
```

---

## ✊ An Open Call to Our People

We must protect our mother tongue at all costs. Not with anger, but with working code.

If you speak Sylheti—whether you are from South Assam (Cachar, Karimganj, Hailakandi), Sylhet, or London—please contribute. Open a PR, add vocabulary from your locality, fine-tune pronunciations, and let's ensure our language never goes silent.

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

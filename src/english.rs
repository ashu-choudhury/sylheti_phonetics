//! Common English vocabulary detector for bilingual code-switching in chat.
//!
//! Preserves standard English words in their original Latin script so that
//! multilingual TTS synthesizers (like Nuance Vocalizer, Google TTS, SAPI) can speak
//! them natively with their English voice module rather than mangling them.

/// Curated set of high-frequency English words commonly used in bilingual chats.
static COMMON_ENGLISH_WORDS: &[&str] = &[
    // Greetings & Social
    "hello", "hi", "hey", "bye", "ok", "okay", "please", "sorry", "thanks", "thank", "welcome",
    "good", "morning", "night", "evening", "afternoon", "congrats", "happy", "birthday",
    // Work, School & Study
    "office", "meeting", "job", "work", "boss", "colleague", "interview", "salary",
    "school", "college", "university", "class", "exam", "test", "result", "teacher",
    "sir", "madam", "student", "study", "project", "presentation", "assignment",
    // Technology & Social Media
    "mobile", "phone", "call", "message", "msg", "chat", "online", "offline", "network",
    "internet", "wifi", "laptop", "computer", "app", "application", "battery", "charge",
    "charger", "video", "audio", "photo", "pic", "camera", "link", "youtube", "facebook",
    "whatsapp", "instagram", "google", "mail", "email", "post", "status", "profile",
    "update", "download", "upload", "share", "install", "file", "document", "data", "live", "location", "number", "money", "news",
    // Transport & Places
    "bus", "train", "car", "bike", "taxi", "auto", "road", "station", "airport", "ticket",
    "seat", "travel", "tour", "hotel", "restaurant", "hospital", "doctor", "medicine",
    "clinic", "pharmacy", "bank", "atm", "market", "shop", "mall",
    // Time & Scheduling
    "now", "later", "today", "tomorrow", "yesterday", "sunday", "monday", "tuesday",
    "wednesday", "thursday", "friday", "saturday", "time", "date", "minute", "hour",
    "week", "month", "year", "wait", "late", "early", "urgent", "busy", "free", "ready",
    // Common Verbs & Adjectives
    "see", "hear", "listen", "tell", "say", "know", "think", "want", "need", "like",
    "love", "hate", "feel", "try", "help", "send", "give", "take", "check", "confirm",
    "cancel", "join", "leave", "start", "stop", "finish", "open", "close", "important",
    "problem", "issue", "idea", "plan", "detail", "details", "reason", "friend", "best",
    "bro", "brother", "sister", "friend", "guy", "guys", "everyone", "anybody", "nobody",
    "sure", "true", "false", "real", "fake", "clear", "easy", "hard", "simple", "direct",
    "fast", "slow", "fresh", "cool", "super", "great", "nice", "fine", "perfect",
];

/// Checks if a lowercase token is a recognized common English word.
pub fn is_english(token: &str) -> bool {
    let lower = token.to_lowercase();
    COMMON_ENGLISH_WORDS.contains(&lower.as_str())
}

/// Checks if a token is an English word with an attached Sylheti grammatical suffix
/// (e.g. "office-o" -> "office" + "-o", "meeting-e" -> "meeting" + "-e").
///
/// Returns `Some((english_stem, suffix))` if matched.
pub fn split_english_suffix(token: &str) -> Option<(&str, &str)> {
    let lower = token.to_lowercase();

    // Check hyphenated forms first: e.g. "office-o"
    if let Some((stem, suf)) = token.split_once('-') {
        if is_english(stem) {
            return Some((stem, suf));
        }
    }

    // Check direct attached suffixes: "-o", "-e", "-ta", "-re", "-or"
    let suffixes = ["-o", "-e", "-ta", "-re", "-or", "o", "e", "ta", "re", "or"];
    for suf in suffixes {
        if lower.ends_with(suf) && lower.len() > suf.len() + 2 {
            let stem_len = token.len() - suf.len();
            let stem = &token[..stem_len];
            if is_english(stem) {
                return Some((stem, suf));
            }
        }
    }

    None
}

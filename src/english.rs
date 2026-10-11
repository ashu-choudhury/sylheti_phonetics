//! Common English vocabulary detector and natural loanword phonetics for bilingual code-switching.

use crate::types::TargetScript;

/// Structure defining an English loanword mapping across scripts.
struct LoanwordEntry {
    english: &'static str,
    hindi: &'static str,
    bengali: &'static str,
    nagri: &'static str,
}

static ENGLISH_LOANWORDS: &[LoanwordEntry] = &[
    LoanwordEntry { english: "office", hindi: "ऑफ़िस", bengali: "অফিস", nagri: "ꠅꠚꠤꠍ" },
    LoanwordEntry { english: "meeting", hindi: "मीटिंग", bengali: "মিটিং", nagri: "ꠝꠤꠐꠤꠋ" },
    LoanwordEntry { english: "call", hindi: "कॉल", bengali: "কল", nagri: "ꠇꠟ" },
    LoanwordEntry { english: "message", hindi: "मैसेज", bengali: "মেসেজ", nagri: "ꠝꠦꠍꠦꠎ" },
    LoanwordEntry { english: "msg", hindi: "मैसेज", bengali: "মেসেজ", nagri: "ꠝꠦꠍꠦꠎ" },
    LoanwordEntry { english: "please", hindi: "प्लीज़", bengali: "প্লিজ", nagri: "ꠙ꠆ꠟꠤꠎ" },
    LoanwordEntry { english: "later", hindi: "लेटर", bengali: "লেটার", nagri: "ꠟꠦꠐꠣꠞ" },
    LoanwordEntry { english: "now", hindi: "नाउ", bengali: "নাউ", nagri: "ꠘꠣꠃ" },
    LoanwordEntry { english: "mobile", hindi: "मोबाइल", bengali: "মোবাইল", nagri: "ꠝꠧꠛꠣꠁꠟ" },
    LoanwordEntry { english: "phone", hindi: "फ़ोन", bengali: "ফোন", nagri: "ꠙꠧꠘ" },
    LoanwordEntry { english: "bus", hindi: "बस", bengali: "বাস", nagri: "ꠛꠣꠍ" },
    LoanwordEntry { english: "train", hindi: "ट्रेन", bengali: "ট্রেন", nagri: "ꠐ꠆ꠞꠦꠘ" },
    LoanwordEntry { english: "hospital", hindi: "हॉस्पिटल", bengali: "হাসপাতাল", nagri: "ꠢꠣꠡꠙꠣꠔꠣꠟ" },
    LoanwordEntry { english: "doctor", hindi: "डॉक्टर", bengali: "ডাক্তার", nagri: "ꠒꠣꠇ꠆ꠔꠣꠞ" },
    LoanwordEntry { english: "college", hindi: "कॉलेज", bengali: "কলেজ", nagri: "ꠇꠟꠦꠎ" },
    LoanwordEntry { english: "school", hindi: "स्कूल", bengali: "স্কুল", nagri: "ꠁꠡ꠆ꠇꠥꠟ" },
    LoanwordEntry { english: "class", hindi: "क्लास", bengali: "ক্লাস", nagri: "ꠇ꠆ꠟꠣꠍ" },
    LoanwordEntry { english: "exam", hindi: "एग्ज़ाम", bengali: "এক্সাম", nagri: "ꠄꠇ꠆ꠎꠣꠝ" },
    LoanwordEntry { english: "test", hindi: "टेस्ट", bengali: "টেস্ট", nagri: "ꠐꠦꠡ꠆ꠐ" },
    LoanwordEntry { english: "result", hindi: "रिजल्ट", bengali: "রেজাল্ট", nagri: "ꠞꠦꠎꠣꠟ꠆ꠐ" },
    LoanwordEntry { english: "time", hindi: "टाइम", bengali: "টাইম", nagri: "ꠐꠣꠁꠝ" },
    LoanwordEntry { english: "ticket", hindi: "टिकट", bengali: "টিকিট", nagri: "ꠐꠤꠇꠤꠐ" },
    LoanwordEntry { english: "check", hindi: "चेक", bengali: "চেক", nagri: "ꠌꠦꠇ" },
    LoanwordEntry { english: "update", hindi: "अपडेट", bengali: "আপডেট", nagri: "ꠀꠙꠒꠦꠐ" },
    LoanwordEntry { english: "important", hindi: "इम्पॉर्टेन्ट", bengali: "ইম্পরট্যান্ট", nagri: "ꠁꠝ꠆ꠙꠞꠐꠦꠘꠐ" },
    LoanwordEntry { english: "problem", hindi: "प्रॉब्लम", bengali: "প্রবলেম", nagri: "ꠙ꠆ꠞꠛꠟꠦꠝ" },
    LoanwordEntry { english: "free", hindi: "फ़्री", bengali: "ফ্রি", nagri: "ꠙ꠆ꠞꠤ" },
    LoanwordEntry { english: "busy", hindi: "बिज़ी", bengali: "বিজি", nagri: "ꠛꠤꠎꠤ" },
    LoanwordEntry { english: "ready", hindi: "रेडी", bengali: "রেডি", nagri: "ꠞꠦꠒꠤ" },
    LoanwordEntry { english: "wait", hindi: "वेट", bengali: "ওয়েট", nagri: "ꠅꠄꠐ" },
    LoanwordEntry { english: "late", hindi: "लेट", bengali: "লেট", nagri: "ꠟꠦꠐ" },
    LoanwordEntry { english: "urgent", hindi: "अर्जेंट", bengali: "আর্জেন্ট", nagri: "ꠀꠞ꠆ꠎꠦꠘ꠆ꠐ" },
    LoanwordEntry { english: "help", hindi: "हेल्प", bengali: "হেল্প", nagri: "ꠢꠦꠟ꠆ꠙ" },
    LoanwordEntry { english: "job", hindi: "जॉब", bengali: "জব", nagri: "ꠎꠛ" },
    LoanwordEntry { english: "work", hindi: "वर्क", bengali: "ওয়ার্ক", nagri: "ꠅꠀꠞ꠆ꠇ" },
    LoanwordEntry { english: "start", hindi: "स्टार्ट", bengali: "স্টার্ট", nagri: "ꠁꠡ꠆ꠐꠣꠞ꠆ꠐ" },
    LoanwordEntry { english: "stop", hindi: "स्टॉप", bengali: "স্টপ", nagri: "ꠁꠡ꠆ꠐꠙ" },
    LoanwordEntry { english: "finish", hindi: "फ़िनिश", bengali: "ফিনিশ", nagri: "ꠙꠤꠘꠤꠡ" },
    LoanwordEntry { english: "cancel", hindi: "कैंसिल", bengali: "ক্যানসেল", nagri: "ꠇꠦꠘ꠆ꠍꠦꠟ" },
    LoanwordEntry { english: "confirm", hindi: "कन्फ़र्म", bengali: "কনফার্ম", nagri: "ꠇꠘꠚꠣꠞ꠆ꠝ" },
    LoanwordEntry { english: "station", hindi: "स्टेशन", bengali: "স্টেশন", nagri: "ꠁꠡ꠆ꠐꠦꠡꠘ" },
    LoanwordEntry { english: "airport", hindi: "एयरपोर्ट", bengali: "এয়ারপোর্ট", nagri: "ꠄꠀꠞꠙꠧꠞ꠆ꠐ" },
    LoanwordEntry { english: "hotel", hindi: "होटल", bengali: "হোটেল", nagri: "ꠢꠧꠐꠦꠟ" },
    LoanwordEntry { english: "bank", hindi: "बैंक", bengali: "ব্যাংক", nagri: "ꠛꠦꠋꠇ" },
    LoanwordEntry { english: "road", hindi: "रोड", bengali: "রোড", nagri: "ꠞꠧꠒ" },
    LoanwordEntry { english: "market", hindi: "मार्केट", bengali: "মার্কেট", nagri: "ꠝꠣꠞ꠆ꠇꠦꠐ" },
    LoanwordEntry { english: "shop", hindi: "शॉप", bengali: "শপ", nagri: "ꠡꠙ" },
    LoanwordEntry { english: "online", hindi: "ऑनलाइन", bengali: "অনলাইন", nagri: "ꠅꠘꠟꠣꠁꠘ" },
    LoanwordEntry { english: "offline", hindi: "ऑफ़लाइन", bengali: "অফলাইন", nagri: "ꠅꠚꠟꠣꠁꠘ" },
    LoanwordEntry { english: "video", hindi: "वीडियो", bengali: "ভিডিও", nagri: "ꠜꠤꠒꠤꠅ" },
    LoanwordEntry { english: "photo", hindi: "फ़ोटो", bengali: "ফটো", nagri: "ꠙꠐꠧ" },
    LoanwordEntry { english: "pic", hindi: "पिक", bengali: "পিক", nagri: "ꠙꠤꠇ" },
    LoanwordEntry { english: "battery", hindi: "बैटरी", bengali: "ব্যাটারি", nagri: "ꠛꠦꠐꠣꠞꠤ" },
    LoanwordEntry { english: "charger", hindi: "चार्जर", bengali: "চার্জার", nagri: "ꠌꠣꠞ꠆ꠎꠣꠞ" },
    LoanwordEntry { english: "charge", hindi: "चार्ज", bengali: "চার্জ", nagri: "ꠌꠣꠞ꠆ꠎ" },
    LoanwordEntry { english: "live", hindi: "लाइव", bengali: "লাইভ", nagri: "ꠟꠣꠁꠜ" },
    LoanwordEntry { english: "location", hindi: "लोकेशन", bengali: "লোকেশন", nagri: "ꠟꠧꠇꠦꠡꠘ" },
    LoanwordEntry { english: "number", hindi: "नंबर", bengali: "নাম্বার", nagri: "ꠘꠣꠝ꠆ꠛꠣꠞ" },
    LoanwordEntry { english: "money", hindi: "मनी", bengali: "মানি", nagri: "ꠝꠣꠘꠤ" },
    LoanwordEntry { english: "news", hindi: "न्यूज़", bengali: "নিউজ", nagri: "ꠘꠤꠃꠎ" },
    LoanwordEntry { english: "link", hindi: "लिंक", bengali: "লিঙ্ক", nagri: "ꠟꠤꠋꠇ" },
    LoanwordEntry { english: "internet", hindi: "इंटरनेट", bengali: "ইন্টারনেট", nagri: "ꠁꠘ꠆ꠐꠣꠞꠘꠦꠐ" },
    LoanwordEntry { english: "wifi", hindi: "वाइफ़ाइ", bengali: "ওয়াইফাই", nagri: "ꠅꠀꠁꠚꠣꠁ" },
    LoanwordEntry { english: "download", hindi: "डाउनलोड", bengali: "ডাউনলোড", nagri: "ꠒꠣꠃꠘꠟꠧꠒ" },
    LoanwordEntry { english: "upload", hindi: "अपलोड", bengali: "আপলোড", nagri: "ꠀꠙꠟꠧꠒ" },
    LoanwordEntry { english: "share", hindi: "शेयर", bengali: "শেয়ার", nagri: "ꠡꠦꠀꠞ" },
    LoanwordEntry { english: "sure", hindi: "श्योर", bengali: "শিওর", nagri: "ꠡꠤꠅꠞ" },
    LoanwordEntry { english: "clear", hindi: "क्लियर", bengali: "ক্লিয়ার", nagri: "ꠇ꠆ꠟꠤꠀꠞ" },
    LoanwordEntry { english: "super", hindi: "सुपर", bengali: "সুপার", nagri: "ꠡꠥꠙꠣꠞ" },
    LoanwordEntry { english: "perfect", hindi: "परफ़ेक्ट", bengali: "পারফেক্ট", nagri: "ꠙꠣꠞꠚꠦꠇ꠆ꠐ" },
    LoanwordEntry { english: "best", hindi: "बेस्ट", bengali: "বেস্ট", nagri: "ꠛꠦꠡ꠆ꠐ" },
    LoanwordEntry { english: "friend", hindi: "फ्रेंड", bengali: "ফ্রেন্ড", nagri: "ꠙ꠆ꠞꠦꠘ꠆ꠒ" },
    LoanwordEntry { english: "details", hindi: "डिटेल्स", bengali: "ডিটেইলস", nagri: "ꠒꠤꠐꠦꠁꠟ꠆ꠍ" },
    LoanwordEntry { english: "detail", hindi: "डिटेल", bengali: "ডিটেইল", nagri: "ꠒꠤꠐꠦꠁꠟ" },
];

/// Looks up an English loanword and returns its natural phonetic transcription in the target script.
pub fn lookup_loanword(token: &str, target: TargetScript) -> Option<&'static str> {
    let lower = token.to_lowercase();
    for entry in ENGLISH_LOANWORDS {
        if entry.english == lower {
            return match target {
                TargetScript::HindiVocalizer => Some(entry.hindi),
                TargetScript::Bengali => Some(entry.bengali),
                TargetScript::SylhetiNagri => Some(entry.nagri),
            };
        }
    }
    None
}

/// Checks if a lowercase token is a recognized common English word or loanword.
pub fn is_english(token: &str) -> bool {
    let lower = token.to_lowercase();
    if lookup_loanword(&lower, TargetScript::HindiVocalizer).is_some() {
        return true;
    }
    // High-performance O(log N) binary search over 10,000 common English words
    crate::english_data::COMMON_ENGLISH_WORDS.binary_search(&lower.as_str()).is_ok()
}

/// Checks if a token is an English word with an attached Sylheti grammatical suffix
/// (e.g. "office-o" -> ("office", "-o"), "meeting-e" -> ("meeting", "-e")).
pub fn split_english_suffix(token: &str) -> Option<(&str, &str)> {
    let lower = token.to_lowercase();

    // Check hyphenated forms first: e.g. "office-o"
    if let Some((stem, suf)) = token.split_once('-').filter(|(stem, _)| is_english(stem)) {
        return Some((stem, suf));
    }

    // Check direct attached suffixes: "-o", "-e", "-ta", "-re", "-or"
    let suffixes = ["o", "e", "ta", "re", "or"];
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

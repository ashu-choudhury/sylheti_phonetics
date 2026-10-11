use sylheti_phonetics::to_hindi_phonetic_strict;

#[test]
fn test_whatsapp_messages_family_chat() {
    // 1. "Change korsoin ni" -> "Change" is English, "korsoin" and "ni" are Sylheti
    let t1 = to_hindi_phonetic_strict("Change korsoin ni");
    println!("t1: {}", t1);
    assert!(t1.contains("Change"));
    assert!(t1.contains("ख़ोरसोइन") || t1.contains("कोरसोइन") || t1.contains("कोरशोइन"));
    assert!(t1.contains("नी"));

    // 2. "Ticket o lekha ai sa ni ?"
    let t2 = to_hindi_phonetic_strict("Ticket o lekha ai sa ni ?");
    println!("t2: {}", t2);
    assert!(t2.contains("Ticket"));
    assert!(t2.contains("लेख़ा") || t2.contains("लेखा"));
    assert!(t2.contains("आइ") || t2.contains("आई"));
    assert!(t2.contains("सा") || t2.contains("शा"));
    assert!(t2.contains("नी"));

    // 3. "ghty taki utita"
    let t3 = to_hindi_phonetic_strict("ghty taki utita");
    println!("t3: {}", t3);
    assert!(t3.contains("घाटी"));
    assert!(t3.contains("ताकी") || t3.contains("थाकी"));
    assert!(t3.contains("उठीता") || t3.contains("उतीता"));

    // 4. "Oi"
    let t4 = to_hindi_phonetic_strict("Oi");
    println!("t4: {}", t4);
    assert!(t4.contains("ओइ"));

    // 5. Desktop preservation check
    let t5 = to_hindi_phonetic_strict("Windows Desktop File Explorer Settings Taskbar Recycle Bin");
    println!("t5: {}", t5);
    assert_eq!(t5, "Windows Desktop File Explorer Settings Taskbar Recycle Bin");
}

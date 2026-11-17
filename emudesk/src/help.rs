//! Turkish RDP usage guide rendered as scrollable text.
//!
//! This is purely static help content shown in the "Yardım" (Help) page. It
//! carries no logic beyond what to display.

/// Title shown in the Help page header.
pub const HELP_TITLE: &str = "Uzak Masaüstü (RDP) Kullanım Rehberi";

/// Lines of help content. Each item is one rendered paragraph/line.
pub const HELP_LINES: &[&str] = &[
    "EmuDesk, Windows'un yerleşik Uzak Masaüstü (Remote Desktop Protocol, RDP) ",
    "istemcisi mstsc.exe'yi başlatarak bağlantı kurar. Aşağıdaki adımlar sorunsuz ",
    "bir deneyim için gereklidir.",
    "",
    "1) Uzak cihazda sürüm gereksinimi",
    "   Bağlanmak istediğiniz bilgisayarda Windows Pro, Enterprise veya Education ",
    "   sürümü (ya da uygun bir Windows Server sürümü) çalışıyor olmalıdır. Windows ",
    "   Home sürümü yerleşik RDP sunucusu barındıramaz.",
    "",
    "2) Uzak cihazda Uzak Masaüstü'nü etkinleştirin",
    "   Uzak cihazda Başlat > Ayarlar > Sistem > Uzak Masaüstü yolunu izleyin ve ",
    "   \"Uzak Masaüstü\" anahtarını açın. İstenirse güvenlik duvarı kuralını ",
    "   onaylayın.",
    "",
    "3) Kullanıcı hesabı ve parola",
    "   Uzak cihazda oturum açabilecek bir kullanıcı hesabı ve bu hesabın parolası ",
    "   gerekir. Bağlantı sırasında Windows kendi kimlik doğrulama ekranını ",
    "   gösterecek ve parolanızı soracaktır. Parolanız hiçbir zaman EmuDesk ",
    "   tarafından saklanmaz veya ağa gönderilmez.",
    "",
    "4) Bağlantı hedefi",
    "   Bağlantı için uzak cihazın IP adresini veya bilgisayar adını kullanabilirsiniz. ",
    "   Aynı ağda (LAN) çalışıyorsanız IP adresi genellikle yeterlidir.",
    "",
    "5) Uyku modu uyarısı",
    "   Uyku veya hazırda bekletme modundaki bir bilgisayara bağlanılamaz. Uzak ",
    "   cihazın açık ve ağa bağlı olduğundan emin olun.",
    "",
    "6) Güvenli ağ bağlantısı",
    "   RDP güvenli bir ağ bağlantısı gerektirir. Güvenilmeyen bir ağdan (ör. internet) ",
    "   bağlanırken VPN kullanın ve bağlantıyı doğrulamak için sertifika uyarılarını ",
    "   dikkate alın.",
    "",
    "7) İstemci sürümü",
    "   Windows Home sürümü Uzak Masaüstü istemcisi olarak sorunsuz kullanılabilir; ",
    "   ancak bu sürüm yerleşik RDP sunucusu olarak kullanılamaz.",
    "",
    "İpucu: Bağlantı kurulamıyorsa hedef IP/bilgisayar adını, güvenlik duvarı ",
    "kurallarını ve uzak cihazın Uzak Masaüstü ayarını kontrol edin.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_has_content() {
        assert!(!HELP_LINES.is_empty());
        assert!(!HELP_TITLE.is_empty());
        // Non-empty lines should not contain NUL bytes or look binary.
        for line in HELP_LINES {
            assert!(!line.contains('\0'));
        }
    }
}

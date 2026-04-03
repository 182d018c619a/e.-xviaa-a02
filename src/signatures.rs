use regex::Regex;

/// Struktur untuk mendefinisikan pola kebocoran data
pub struct ScannerSignature {
    pub name: &'static str,
    pub description: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
}

/// Level bahaya dari temuan
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

/// Fungsi untuk memuat semua database tanda-tanda kebocoran data
pub fn load_signatures() -> Vec<ScannerSignature> {
    vec![
        // 1. CLOUD INFRASTRUCTURE
        ScannerSignature {
            name: "AWS Access Key ID",
            description: "Identitas akses AWS Cloud ditemukan",
            pattern: r"(?i)AKIA[0-9A-Z]{16}",
            severity: Severity::Critical,
        },
        ScannerSignature {
            name: "Google API Key",
            description: "Google Cloud Platform API Key terdeteksi",
            pattern: r"AIza[0-9A-Za-z\\-_]{35}",
            severity: Severity::High,
        },
        // 2. DATABASE & CONFIGURATION
        ScannerSignature {
            name: "Database Connection String",
            description: "Kredensial database (User/Pass) terekspos",
            pattern: r"(mongodb(?:\+srv)?|postgres|mysql)://[a-zA-Z0-9_]+:[a-zA-Z0-9_]+@",
            severity: Severity::Critical,
        },
        ScannerSignature {
            name: "Environment Variable",
            description: "Variabel lingkungan sensitif (DB_PASSWORD/SECRET)",
            pattern: r"(?i)(DB_PASS|DATABASE_URL|SECRET_KEY|APP_KEY)=",
            severity: Severity::High,
        },
        // 3. PRIVATE KEYS & AUTH
        ScannerSignature {
            name: "RSA Private Key",
            description: "Kunci privat SSH/RSA ditemukan",
            pattern: r"-----BEGIN RSA PRIVATE KEY-----",
            severity: Severity::Critical,
        },
        ScannerSignature {
            name: "Firebase URL",
            description: "Database Firebase terekspos",
            pattern: r"https://[a-zA-Z0-9-]+\.firebaseio\.com",
            severity: Severity::Medium,
        },
    ]
}

/// Fungsi logika untuk melakukan pengecekan konten secara mendalam
pub fn scan_content(body: &str) -> Vec<(&str, Severity)> {
    let signatures = load_signatures();
    let mut findings = Vec::new();

    for sig in signatures {
        let re = Regex::new(sig.pattern).unwrap();
        if re.is_match(body) {
            findings.push((sig.name, sig.severity));
        }
    }
    findings
}

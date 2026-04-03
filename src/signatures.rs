use regex::Regex;
use once_cell::sync::Lazy;

/// Level bahaya dari temuan
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

/// Struktur untuk mendefinisikan pola kebocoran data
#[derive(Debug)]
pub struct ScannerSignature {
    pub name: &'static str,
    pub description: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
}

/// Versi compiled agar tidak compile regex berulang
pub struct CompiledSignature {
    pub name: &'static str,
    pub description: &'static str,
    pub regex: Regex,
    pub severity: Severity,
}

/// Struktur hasil temuan (lebih detail)
#[derive(Debug)]
pub struct Finding {
    pub name: &'static str,
    pub description: &'static str,
    pub severity: Severity,
    pub matched_value: String,
    pub start: usize,
    pub end: usize,
}

/// Database signature mentah
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
            pattern: r"AIza[0-9A-Za-z\-_]{35}",
            severity: Severity::High,
        },

        // 2. DATABASE & CONFIGURATION
        ScannerSignature {
            name: "Database Connection String",
            description: "Kredensial database (User/Pass) terekspos",
            pattern: r"(mongodb(?:\+srv)?|postgres|mysql)://[a-zA-Z0-9_]+:[^@\s]+@",
            severity: Severity::Critical,
        },
        ScannerSignature {
            name: "Environment Variable",
            description: "Variabel lingkungan sensitif",
            pattern: r"(?i)(DB_PASS|DATABASE_URL|SECRET_KEY|APP_KEY)=.+",
            severity: Severity::High,
        },

        // 3. PRIVATE KEYS & AUTH
        ScannerSignature {
            name: "RSA Private Key",
            description: "Kunci privat RSA ditemukan",
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

/// Lazy static compiled regex (thread-safe, compile sekali)
pub static COMPILED_SIGNATURES: Lazy<Vec<CompiledSignature>> = Lazy::new(|| {
    load_signatures()
        .into_iter()
        .filter_map(|sig| {
            match Regex::new(sig.pattern) {
                Ok(regex) => Some(CompiledSignature {
                    name: sig.name,
                    description: sig.description,
                    regex,
                    severity: sig.severity,
                }),
                Err(err) => {
                    eprintln!("Regex error in {}: {}", sig.name, err);
                    None
                }
            }
        })
        .collect()
});

/// Fungsi utama scanning (deep scan)
pub fn scan_content(body: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for sig in COMPILED_SIGNATURES.iter() {
        for mat in sig.regex.find_iter(body) {
            findings.push(Finding {
                name: sig.name,
                description: sig.description,
                severity: sig.severity,
                matched_value: mat.as_str().to_string(),
                start: mat.start(),
                end: mat.end(),
            });
        }
    }

    findings
}

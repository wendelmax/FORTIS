//! Utilitários de validação para o FORTIS Backend

use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Validador de CPF brasileiro
pub struct CpfValidator;

impl CpfValidator {
    /// Valida se um CPF é válido
    pub fn is_valid(cpf: &str) -> bool {
        let cpf = cpf.replace(&['.', '-', ' '][..], "");
        
        if cpf.len() != 11 {
            return false;
        }
        
        if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
            return false;
        }
        
        let digits: Vec<u32> = cpf.chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();
        
        // Validação do primeiro dígito verificador
        let mut sum = 0;
        for i in 0..9 {
            sum += digits[i] * (10 - i as u32);
        }
        let remainder = sum % 11;
        let first_digit = if remainder < 2 { 0 } else { 11 - remainder };
        
        if digits[9] != first_digit {
            return false;
        }
        
        // Validação do segundo dígito verificador
        let mut sum = 0;
        for i in 0..10 {
            sum += digits[i] * (11 - i as u32);
        }
        let remainder = sum % 11;
        let second_digit = if remainder < 2 { 0 } else { 11 - remainder };
        
        digits[10] == second_digit
    }
    
    /// Formata um CPF
    pub fn format(cpf: &str) -> String {
        let cpf = cpf.replace(&['.', '-', ' '][..], "");
        if cpf.len() == 11 {
            format!("{}.{}.{}-{}", &cpf[0..3], &cpf[3..6], &cpf[6..9], &cpf[9..11])
        } else {
            cpf.to_string()
        }
    }
}

/// Validador de CNPJ brasileiro
pub struct CnpjValidator;

impl CnpjValidator {
    /// Valida se um CNPJ é válido
    pub fn is_valid(cnpj: &str) -> bool {
        let cnpj = cnpj.replace(&['.', '-', '/', ' '][..], "");
        
        if cnpj.len() != 14 {
            return false;
        }
        
        if cnpj.chars().all(|c| c == cnpj.chars().next().unwrap()) {
            return false;
        }
        
        let digits: Vec<u32> = cnpj.chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();
        
        // Validação do primeiro dígito verificador
        let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut sum = 0;
        for i in 0..12 {
            sum += digits[i] * weights1[i];
        }
        let remainder = sum % 11;
        let first_digit = if remainder < 2 { 0 } else { 11 - remainder };
        
        if digits[12] != first_digit {
            return false;
        }
        
        // Validação do segundo dígito verificador
        let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
        let mut sum = 0;
        for i in 0..13 {
            sum += digits[i] * weights2[i];
        }
        let remainder = sum % 11;
        let second_digit = if remainder < 2 { 0 } else { 11 - remainder };
        
        digits[13] == second_digit
    }
    
    /// Formata um CNPJ
    pub fn format(cnpj: &str) -> String {
        let cnpj = cnpj.replace(&['.', '-', '/', ' '][..], "");
        if cnpj.len() == 14 {
            format!("{}.{}.{}/{}-{}", 
                &cnpj[0..2], &cnpj[2..5], &cnpj[5..8], 
                &cnpj[8..12], &cnpj[12..14])
        } else {
            cnpj.to_string()
        }
    }
}

/// Validador de email
pub struct EmailValidator;

impl EmailValidator {
    /// Valida se um email é válido
    pub fn is_valid(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
    
    /// Valida se um email é válido e não é um email temporário
    pub fn is_valid_and_permanent(email: &str) -> bool {
        if !Self::is_valid(email) {
            return false;
        }
        
        let temporary_domains = [
            "10minutemail.com",
            "tempmail.org",
            "guerrillamail.com",
            "mailinator.com",
            "temp-mail.org",
        ];
        
        let domain = email.split('@').nth(1).unwrap_or("");
        !temporary_domains.contains(&domain)
    }
}

/// Validador de telefone brasileiro
pub struct PhoneValidator;

impl PhoneValidator {
    /// Valida se um telefone é válido
    pub fn is_valid(phone: &str) -> bool {
        let phone = phone.replace(&['(', ')', '-', ' ', '+'][..], "");
        
        if phone.len() < 10 || phone.len() > 11 {
            return false;
        }
        
        phone.chars().all(|c| c.is_ascii_digit())
    }
    
    /// Formata um telefone
    pub fn format(phone: &str) -> String {
        let phone = phone.replace(&['(', ')', '-', ' ', '+'][..], "");
        
        if phone.len() == 11 {
            format!("({}) {}-{}", &phone[0..2], &phone[2..7], &phone[7..11])
        } else if phone.len() == 10 {
            format!("({}) {}-{}", &phone[0..2], &phone[2..6], &phone[6..10])
        } else {
            phone.to_string()
        }
    }
}

/// Validador de CEP brasileiro
pub struct CepValidator;

impl CepValidator {
    /// Valida se um CEP é válido
    pub fn is_valid(cep: &str) -> bool {
        let cep = cep.replace(&['-', ' '][..], "");
        
        if cep.len() != 8 {
            return false;
        }
        
        cep.chars().all(|c| c.is_ascii_digit())
    }
    
    /// Formata um CEP
    pub fn format(cep: &str) -> String {
        let cep = cep.replace(&['-', ' '][..], "");
        if cep.len() == 8 {
            format!("{}-{}", &cep[0..5], &cep[5..8])
        } else {
            cep.to_string()
        }
    }
}

/// Validador de data
pub struct DateValidator;

impl DateValidator {
    /// Valida se uma data é válida
    pub fn is_valid(date: &str) -> bool {
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
    }
    
    /// Valida se uma data é válida e não é futura
    pub fn is_valid_and_not_future(date: &str) -> bool {
        if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            parsed_date <= chrono::Utc::now().date_naive()
        } else {
            false
        }
    }
    
    /// Valida se uma data é válida e não é muito antiga
    pub fn is_valid_and_not_too_old(date: &str, max_age_years: i32) -> bool {
        if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            let now = chrono::Utc::now().date_naive();
            let max_date = now - chrono::Duration::days(max_age_years as i64 * 365);
            parsed_date >= max_date
        } else {
            false
        }
    }
}

/// Validador de senha
pub struct PasswordValidator;

impl PasswordValidator {
    /// Valida se uma senha é forte
    pub fn is_strong(password: &str) -> bool {
        if password.len() < 8 {
            return false;
        }
        
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        has_uppercase && has_lowercase && has_digit && has_special
    }
    
    /// Valida se uma senha atende aos requisitos mínimos
    pub fn meets_minimum_requirements(password: &str) -> bool {
        password.len() >= 8 && password.chars().any(|c| c.is_ascii_digit())
    }
}

/// Validador de JSON
pub struct JsonValidator;

impl JsonValidator {
    /// Valida se um JSON é válido
    pub fn is_valid(json_str: &str) -> bool {
        serde_json::from_str::<Value>(json_str).is_ok()
    }
    
    /// Valida se um JSON tem a estrutura esperada
    pub fn has_required_fields(json_str: &str, required_fields: &[&str]) -> bool {
        if let Ok(json) = serde_json::from_str::<Value>(json_str) {
            if let Some(obj) = json.as_object() {
                required_fields.iter().all(|field| obj.contains_key(*field))
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Validador de URL
pub struct UrlValidator;

impl UrlValidator {
    /// Valida se uma URL é válida
    pub fn is_valid(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
    
    /// Valida se uma URL é HTTPS
    pub fn is_https(url: &str) -> bool {
        if let Ok(parsed_url) = url::Url::parse(url) {
            parsed_url.scheme() == "https"
        } else {
            false
        }
    }
}

/// Validador de UUID
pub struct UuidValidator;

impl UuidValidator {
    /// Valida se uma string é um UUID válido
    pub fn is_valid(uuid_str: &str) -> bool {
        uuid::Uuid::parse_str(uuid_str).is_ok()
    }
}

/// Validador de IP
pub struct IpValidator;

impl IpValidator {
    /// Valida se uma string é um IP válido
    pub fn is_valid(ip_str: &str) -> bool {
        ip_str.parse::<std::net::IpAddr>().is_ok()
    }
    
    /// Valida se um IP é IPv4
    pub fn is_ipv4(ip_str: &str) -> bool {
        ip_str.parse::<std::net::Ipv4Addr>().is_ok()
    }
    
    /// Valida se um IP é IPv6
    pub fn is_ipv6(ip_str: &str) -> bool {
        ip_str.parse::<std::net::Ipv6Addr>().is_ok()
    }
}

/// Validador de base64
pub struct Base64Validator;

impl Base64Validator {
    /// Valida se uma string é base64 válido
    pub fn is_valid(base64_str: &str) -> bool {
        base64::decode(base64_str).is_ok()
    }
}

/// Validador de hex
pub struct HexValidator;

impl HexValidator {
    /// Valida se uma string é hex válido
    pub fn is_valid(hex_str: &str) -> bool {
        hex_str.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// Validador de tamanho de string
pub struct StringLengthValidator;

impl StringLengthValidator {
    /// Valida se uma string tem o tamanho correto
    pub fn is_valid_length(text: &str, min_length: usize, max_length: usize) -> bool {
        text.len() >= min_length && text.len() <= max_length
    }
    
    /// Valida se uma string não está vazia
    pub fn is_not_empty(text: &str) -> bool {
        !text.trim().is_empty()
    }
}

/// Validador de caracteres especiais
pub struct SpecialCharValidator;

impl SpecialCharValidator {
    /// Valida se uma string contém apenas caracteres permitidos
    pub fn contains_only_allowed_chars(text: &str, allowed_chars: &str) -> bool {
        text.chars().all(|c| allowed_chars.contains(c))
    }
    
    /// Valida se uma string contém caracteres perigosos
    pub fn contains_dangerous_chars(text: &str) -> bool {
        let dangerous_chars = ['<', '>', '"', '\'', '&', ';', '(', ')', '|', '`', '$'];
        text.chars().any(|c| dangerous_chars.contains(&c))
    }
}

/// Validador de eleição
pub struct ElectionValidator;

impl ElectionValidator {
    /// Valida se uma eleição é válida
    pub fn is_valid(election_data: &Value) -> bool {
        if let Some(obj) = election_data.as_object() {
            obj.contains_key("name") &&
            obj.contains_key("start_date") &&
            obj.contains_key("end_date") &&
            Self::is_valid_name(obj.get("name").and_then(|v| v.as_str()).unwrap_or("")) &&
            Self::is_valid_date_range(
                obj.get("start_date").and_then(|v| v.as_str()).unwrap_or(""),
                obj.get("end_date").and_then(|v| v.as_str()).unwrap_or("")
            )
        } else {
            false
        }
    }
    
    /// Valida se o nome da eleição é válido
    pub fn is_valid_name(name: &str) -> bool {
        StringLengthValidator::is_valid_length(name, 3, 100) &&
        !SpecialCharValidator::contains_dangerous_chars(name)
    }
    
    /// Valida se o intervalo de datas é válido
    pub fn is_valid_date_range(start_date: &str, end_date: &str) -> bool {
        if let (Ok(start), Ok(end)) = (
            chrono::DateTime::parse_from_rfc3339(start_date),
            chrono::DateTime::parse_from_rfc3339(end_date)
        ) {
            start < end
        } else {
            false
        }
    }
}

/// Validador de voto
pub struct VoteValidator;

impl VoteValidator {
    /// Valida se um voto é válido
    pub fn is_valid(vote_data: &Value) -> bool {
        if let Some(obj) = vote_data.as_object() {
            obj.contains_key("election_id") &&
            obj.contains_key("candidate_id") &&
            obj.contains_key("biometric_verification") &&
            UuidValidator::is_valid(obj.get("election_id").and_then(|v| v.as_str()).unwrap_or("")) &&
            UuidValidator::is_valid(obj.get("candidate_id").and_then(|v| v.as_str()).unwrap_or("")) &&
            Base64Validator::is_valid(obj.get("biometric_verification").and_then(|v| v.as_str()).unwrap_or(""))
        } else {
            false
        }
    }
}

/// Validador de candidato
pub struct CandidateValidator;

impl CandidateValidator {
    /// Valida se um candidato é válido
    pub fn is_valid(candidate_data: &Value) -> bool {
        if let Some(obj) = candidate_data.as_object() {
            obj.contains_key("name") &&
            obj.contains_key("election_id") &&
            Self::is_valid_name(obj.get("name").and_then(|v| v.as_str()).unwrap_or("")) &&
            UuidValidator::is_valid(obj.get("election_id").and_then(|v| v.as_str()).unwrap_or(""))
        } else {
            false
        }
    }
    
    /// Valida se o nome do candidato é válido
    pub fn is_valid_name(name: &str) -> bool {
        StringLengthValidator::is_valid_length(name, 2, 100) &&
        !SpecialCharValidator::contains_dangerous_chars(name)
    }
}

/// Validador de auditoria
pub struct AuditValidator;

impl AuditValidator {
    /// Valida se uma auditoria é válida
    pub fn is_valid(audit_data: &Value) -> bool {
        if let Some(obj) = audit_data.as_object() {
            obj.contains_key("election_id") &&
            obj.contains_key("audit_type") &&
            UuidValidator::is_valid(obj.get("election_id").and_then(|v| v.as_str()).unwrap_or("")) &&
            Self::is_valid_audit_type(obj.get("audit_type").and_then(|v| v.as_str()).unwrap_or(""))
        } else {
            false
        }
    }
    
    /// Valida se o tipo de auditoria é válido
    pub fn is_valid_audit_type(audit_type: &str) -> bool {
        matches!(audit_type, "full" | "partial" | "random" | "targeted")
    }
}

/// Validador de configuração
pub struct ConfigValidator;

impl ConfigValidator {
    /// Valida se uma configuração é válida
    pub fn is_valid(config_data: &Value) -> bool {
        if let Some(obj) = config_data.as_object() {
            obj.contains_key("server") &&
            obj.contains_key("database") &&
            obj.contains_key("security") &&
            Self::is_valid_server_config(obj.get("server").unwrap()) &&
            Self::is_valid_database_config(obj.get("database").unwrap()) &&
            Self::is_valid_security_config(obj.get("security").unwrap())
        } else {
            false
        }
    }
    
    /// Valida configuração do servidor
    pub fn is_valid_server_config(server_config: &Value) -> bool {
        if let Some(obj) = server_config.as_object() {
            obj.contains_key("host") &&
            obj.contains_key("port") &&
            IpValidator::is_valid(obj.get("host").and_then(|v| v.as_str()).unwrap_or("")) &&
            obj.get("port").and_then(|v| v.as_u64()).map_or(false, |p| p > 0 && p < 65536)
        } else {
            false
        }
    }
    
    /// Valida configuração do banco de dados
    pub fn is_valid_database_config(database_config: &Value) -> bool {
        if let Some(obj) = database_config.as_object() {
            obj.contains_key("url") &&
            UrlValidator::is_valid(obj.get("url").and_then(|v| v.as_str()).unwrap_or(""))
        } else {
            false
        }
    }
    
    /// Valida configuração de segurança
    pub fn is_valid_security_config(security_config: &Value) -> bool {
        if let Some(obj) = security_config.as_object() {
            obj.contains_key("jwt_secret") &&
            obj.contains_key("encryption_key") &&
            StringLengthValidator::is_valid_length(
                obj.get("jwt_secret").and_then(|v| v.as_str()).unwrap_or(""),
                32,
                256
            ) &&
            StringLengthValidator::is_valid_length(
                obj.get("encryption_key").and_then(|v| v.as_str()).unwrap_or(""),
                32,
                256
            )
        } else {
            false
        }
    }
}

/// Validador de dados de entrada
pub struct InputValidator;

impl InputValidator {
    /// Valida dados de entrada de uma requisição
    pub fn validate_request_data(data: &Value, required_fields: &[&str]) -> Result<(), String> {
        if let Some(obj) = data.as_object() {
            for field in required_fields {
                if !obj.contains_key(*field) {
                    return Err(format!("Campo obrigatório ausente: {}", field));
                }
            }
            
            // Valida tipos de dados
            for (key, value) in obj {
                match key.as_str() {
                    "cpf" => {
                        if let Some(cpf) = value.as_str() {
                            if !CpfValidator::is_valid(cpf) {
                                return Err(format!("CPF inválido: {}", cpf));
                            }
                        }
                    },
                    "email" => {
                        if let Some(email) = value.as_str() {
                            if !EmailValidator::is_valid(email) {
                                return Err(format!("Email inválido: {}", email));
                            }
                        }
                    },
                    "phone" => {
                        if let Some(phone) = value.as_str() {
                            if !PhoneValidator::is_valid(phone) {
                                return Err(format!("Telefone inválido: {}", phone));
                            }
                        }
                    },
                    "date" => {
                        if let Some(date) = value.as_str() {
                            if !DateValidator::is_valid(date) {
                                return Err(format!("Data inválida: {}", date));
                            }
                        }
                    },
                    _ => {}
                }
            }
            
            Ok(())
        } else {
            Err("Dados de entrada devem ser um objeto JSON".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpf_validator() {
        assert!(CpfValidator::is_valid("12345678901"));
        assert!(CpfValidator::is_valid("123.456.789-01"));
        assert!(!CpfValidator::is_valid("12345678900"));
        assert!(!CpfValidator::is_valid("11111111111"));
    }

    #[test]
    fn test_email_validator() {
        assert!(EmailValidator::is_valid("test@example.com"));
        assert!(!EmailValidator::is_valid("invalid-email"));
        assert!(!EmailValidator::is_valid("test@"));
    }

    #[test]
    fn test_password_validator() {
        assert!(PasswordValidator::is_strong("Password123!"));
        assert!(!PasswordValidator::is_strong("password"));
        assert!(!PasswordValidator::is_strong("12345678"));
    }

    #[test]
    fn test_date_validator() {
        assert!(DateValidator::is_valid("2025-01-01"));
        assert!(!DateValidator::is_valid("invalid-date"));
        assert!(DateValidator::is_valid_and_not_future("2020-01-01"));
    }
}

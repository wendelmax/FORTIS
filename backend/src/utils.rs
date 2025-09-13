//! Utilitários do FORTIS Backend

use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Gerar ID único
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Gerar timestamp atual
pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

/// Validar email
pub fn is_valid_email(email: &str) -> bool {
    use regex::Regex;
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// Validar CPF
pub fn is_valid_cpf(cpf: &str) -> bool {
    let cpf_clean = cpf.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    
    if cpf_clean.len() != 11 {
        return false;
    }
    
    if cpf_clean.chars().all(|c| c == cpf_clean.chars().next().unwrap()) {
        return false;
    }
    
    let digits: Vec<u32> = cpf_clean.chars().map(|c| c.to_digit(10).unwrap()).collect();
    
    // Primeiro dígito verificador
    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (10 - i as u32);
    }
    let first_digit = (sum * 10) % 11;
    let first_digit = if first_digit == 10 { 0 } else { first_digit };
    
    if first_digit != digits[9] {
        return false;
    }
    
    // Segundo dígito verificador
    let mut sum = 0;
    for i in 0..10 {
        sum += digits[i] * (11 - i as u32);
    }
    let second_digit = (sum * 10) % 11;
    let second_digit = if second_digit == 10 { 0 } else { second_digit };
    
    second_digit == digits[10]
}

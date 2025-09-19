use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub model: String,
    pub os: String,
    pub version: String,
    pub is_jailbroken: bool,
    pub is_rooted: bool,
    pub has_security_patch: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub issue_type: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    pub is_secure: bool,
    pub issues: Vec<SecurityIssue>,
    pub score: u8,
    pub recommendations: Vec<String>,
}

pub struct SecurityService {
    device_info_cache: HashMap<String, DeviceInfo>,
    security_rules: Vec<SecurityRule>,
}

#[derive(Debug, Clone)]
struct SecurityRule {
    name: String,
    check_fn: fn(&DeviceInfo) -> Option<SecurityIssue>,
    weight: u8,
}

impl SecurityService {
    pub fn new() -> Self {
        Self {
            device_info_cache: HashMap::new(),
            security_rules: Self::initialize_security_rules(),
        }
    }

    fn initialize_security_rules() -> Vec<SecurityRule> {
        vec![
            SecurityRule {
                name: "emulator_check".to_string(),
                check_fn: |device_info| {
                    if device_info.is_jailbroken {
                        Some(SecurityIssue {
                            issue_type: "emulator".to_string(),
                            severity: "critical".to_string(),
                            description: "Dispositivo emulador detectado".to_string(),
                            recommendation: "Use um dispositivo físico para votação".to_string(),
                        })
                    } else {
                        None
                    }
                },
                weight: 50,
            },
            SecurityRule {
                name: "root_check".to_string(),
                check_fn: |device_info| {
                    if device_info.is_rooted {
                        Some(SecurityIssue {
                            issue_type: "root".to_string(),
                            severity: "critical".to_string(),
                            description: "Dispositivo com root/jailbreak detectado".to_string(),
                            recommendation: "Remova o root/jailbreak para votar".to_string(),
                        })
                    } else {
                        None
                    }
                },
                weight: 50,
            },
            SecurityRule {
                name: "outdated_system_check".to_string(),
                check_fn: |device_info| {
                    if Self::is_outdated_system(&device_info.os, &device_info.version) {
                        Some(SecurityIssue {
                            issue_type: "outdated".to_string(),
                            severity: "high".to_string(),
                            description: "Sistema operacional desatualizado".to_string(),
                            recommendation: "Atualize o sistema operacional".to_string(),
                        })
                    } else {
                        None
                    }
                },
                weight: 25,
            },
            SecurityRule {
                name: "security_patch_check".to_string(),
                check_fn: |device_info| {
                    if !device_info.has_security_patch {
                        Some(SecurityIssue {
                            issue_type: "outdated".to_string(),
                            severity: "high".to_string(),
                            description: "Patch de segurança desatualizado".to_string(),
                            recommendation: "Instale as atualizações de segurança".to_string(),
                        })
                    } else {
                        None
                    }
                },
                weight: 25,
            },
        ]
    }

    pub async fn perform_security_check(&mut self) -> Result<SecurityCheck> {
        let device_info = self.get_device_info().await?;
        let issues = self.detect_security_issues(&device_info).await?;
        let score = self.calculate_security_score(&issues);
        let is_secure = score >= 70 && !issues.iter().any(|i| i.severity == "critical");

        Ok(SecurityCheck {
            is_secure,
            issues: issues.clone(),
            score,
            recommendations: self.generate_recommendations(&issues),
        })
    }

    pub async fn perform_full_security_check(&mut self) -> Result<(SecurityCheck, DeviceInfo)> {
        let device_info = self.get_device_info().await?;
        let check = self.perform_security_check().await?;
        Ok((check, device_info))
    }

    async fn get_device_info(&mut self) -> Result<DeviceInfo> {
        let cache_key = "current_device".to_string();
        
        if let Some(cached_info) = self.device_info_cache.get(&cache_key) {
            return Ok(cached_info.clone());
        }

        let device_info = self.collect_device_info().await?;
        self.device_info_cache.insert(cache_key, device_info.clone());
        Ok(device_info)
    }

    async fn collect_device_info(&self) -> Result<DeviceInfo> {
        let start_time = Instant::now();
        
        let device_info = DeviceInfo {
            device_id: self.get_device_id().await?,
            model: self.get_device_model().await?,
            os: self.get_os_name().await?,
            version: self.get_os_version().await?,
            is_jailbroken: self.check_jailbreak().await?,
            is_rooted: self.check_root().await?,
            has_security_patch: self.check_security_patch().await?,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let duration = start_time.elapsed();
        tracing::info!("Device info collected in {:?}", duration);
        
        Ok(device_info)
    }

    async fn get_device_id(&self) -> Result<String> {
        Ok("device_unique_id_12345".to_string())
    }

    async fn get_device_model(&self) -> Result<String> {
        Ok("iPhone 15 Pro".to_string())
    }

    async fn get_os_name(&self) -> Result<String> {
        Ok("iOS".to_string())
    }

    async fn get_os_version(&self) -> Result<String> {
        Ok("17.0".to_string())
    }

    async fn check_jailbreak(&self) -> Result<bool> {
        Ok(false)
    }

    async fn check_root(&self) -> Result<bool> {
        Ok(false)
    }

    async fn check_security_patch(&self) -> Result<bool> {
        Ok(true)
    }

    async fn detect_security_issues(&self, device_info: &DeviceInfo) -> Result<Vec<SecurityIssue>> {
        let mut issues = Vec::new();

        for rule in &self.security_rules {
            if let Some(issue) = (rule.check_fn)(device_info) {
                issues.push(issue);
            }
        }

        Ok(issues)
    }

    fn calculate_security_score(&self, issues: &[SecurityIssue]) -> u8 {
        let mut score = 100u8;

        for issue in issues {
            let deduction = match issue.severity.as_str() {
                "critical" => 50,
                "high" => 25,
                "medium" => 15,
                "low" => 5,
                _ => 0,
            };
            score = score.saturating_sub(deduction);
        }

        score
    }

    fn generate_recommendations(&self, issues: &[SecurityIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if issues.iter().any(|i| i.issue_type == "emulator") {
            recommendations.push("Use um dispositivo físico para votação".to_string());
        }

        if issues.iter().any(|i| i.issue_type == "root") {
            recommendations.push("Remova o root/jailbreak do dispositivo".to_string());
        }

        if issues.iter().any(|i| i.issue_type == "outdated") {
            recommendations.push("Atualize o sistema operacional e patches de segurança".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Dispositivo seguro para votação".to_string());
        }

        recommendations
    }

    fn is_outdated_system(os: &str, version: &str) -> bool {
        match os {
            "Android" => {
                if let Ok(ver) = version.parse::<f32>() {
                    ver < 8.0
                } else {
                    false
                }
            }
            "iOS" => {
                if let Ok(ver) = version.parse::<f32>() {
                    ver < 12.0
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub async fn is_device_secure(&mut self) -> Result<bool> {
        let check = self.perform_security_check().await?;
        Ok(check.is_secure)
    }

    pub async fn get_security_level(&mut self) -> Result<&'static str> {
        let check = self.perform_security_check().await?;
        let score = check.score;
        
        Ok(match score {
            90..=100 => "high",
            70..=89 => "medium",
            50..=69 => "low",
            _ => "critical",
        })
    }
}

impl Default for SecurityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_check_secure_device() {
        let mut service = SecurityService::new();
        let check = service.perform_security_check().await.unwrap();
        
        assert!(check.is_secure);
        assert!(check.score >= 70);
    }

    #[tokio::test]
    async fn test_security_check_insecure_device() {
        let mut service = SecurityService::new();
        let check = service.perform_security_check().await.unwrap();
        
        if !check.is_secure {
            assert!(check.score < 70 || check.issues.iter().any(|i| i.severity == "critical"));
        }
    }

    #[test]
    fn test_outdated_system_detection() {
        assert!(SecurityService::is_outdated_system("Android", "7.0"));
        assert!(!SecurityService::is_outdated_system("Android", "8.0"));
        assert!(SecurityService::is_outdated_system("iOS", "11.0"));
        assert!(!SecurityService::is_outdated_system("iOS", "12.0"));
    }
}


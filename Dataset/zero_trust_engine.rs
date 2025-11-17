use std::collections::{HashMap, HashSet};
use std::fmt;
use std::cmp::max;

// --- 1. Definições de Enums e Structs (Modelagem de Entidades) ---

/// Representa o nível de risco associado a um componente.
/// Quanto maior o valor, maior o risco.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum RiskLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl fmt::Display for RiskLevel {
    /// Permite que RiskLevel seja facilmente impresso como string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Define os privilégios, o risco máximo permitido e a pontuação comportamental do Usuário.
#[derive(Debug, Clone)]
struct UserIdentity {
    user_id: String,
    role: String,
    max_risk: RiskLevel,
    allowed_resources: HashSet<String>,
    // Pontuação comportamental (0-100). Pontuações abaixo de 50 são consideradas de alto risco.
    behavioral_score: u8, 
}

/// Representa a postura de segurança do dispositivo.
#[derive(Debug, Clone)]
struct DevicePosture {
    device_id: String,
    is_encrypted: bool,
    patch_status: String, // Ex: "Latest", "Minor_Patch", "Outdated"
    risk: RiskLevel,
}

/// Estrutura que encapsula todos os dados de uma solicitação de acesso em tempo real.
/// Isso é o contexto completo que o Policy Engine avaliará.
#[derive(Debug)]
struct AccessRequest<'a> {
    user_id: &'a str,
    device_id: &'a str,
    source_ip: &'a str,
    resource: &'a str,
    // Simula a hora do dia da solicitação (0 a 23)
    access_hour: u8, 
}

/// Resultado detalhado da decisão do Policy Engine.
#[derive(Debug, PartialEq)]
enum AccessDecision {
    Permitted { session_token: String, effective_risk: RiskLevel },
    Denied(String),
}

// --- 2. Policy Engine (O "Cérebro" Zero Trust) ---

struct PolicyEngine {
    user_database: HashMap<String, UserIdentity>,
    device_database: HashMap<String, DevicePosture>,
    threat_intel: HashMap<String, RiskLevel>,
    // Regras de acesso baseadas no horário (ex: Desenvolvedores não podem acessar API fora do horário comercial)
    time_policies: HashMap<String, (u8, u8)>, // (role, (start_hour, end_hour))
}

impl PolicyEngine {
    /// Construtor para inicializar o Engine com dados simulados e complexos.
    fn new() -> Self {
        let mut user_db = HashMap::new();
        user_db.insert("alice".to_string(), UserIdentity {
            user_id: "alice".to_string(),
            role: "Developer".to_string(),
            max_risk: RiskLevel::Medium,
            allowed_resources: vec!["API_DEV_V1", "GIT_REPO", "LOG_SERVER"].into_iter().map(String::from).collect(),
            behavioral_score: 85, // Bom comportamento
        });
        user_db.insert("bob".to_string(), UserIdentity {
            user_id: "bob".to_string(),
            role: "Sales".toing(),
            max_risk: RiskLevel::Low,
            allowed_resources: vec!["CRM_PROD", "SALES_REPORTS"].into_iter().map(String::from).collect(),
            behavioral_score: 72, // Comportamento aceitável
        });
         user_db.insert("carol".to_string(), UserIdentity {
            user_id: "carol".to_string(),
            role: "Finance".to_string(),
            max_risk: RiskLevel::High,
            allowed_resources: vec!["FINANCE_DB", "HR_PORTAL"].into_iter().map(String::from).collect(),
            behavioral_score: 45, // Comportamento incomum/Suspeito
        });

        let mut device_db = HashMap::new();
        device_db.insert("D_A_123".to_string(), DevicePosture {
            device_id: "D_A_123".to_string(),
            is_encrypted: true,
            patch_status: "Latest".to_string(),
            risk: RiskLevel::Low,
        });
        device_db.insert("D_B_456".to_string(), DevicePosture {
            device_id: "D_B_456".to_string(),
            is_encrypted: false,
            patch_status: "Outdated".to_string(),
            risk: RiskLevel::Medium,
        });
        device_db.insert("D_C_789".to_string(), DevicePosture {
            device_id: "D_C_789".to_string(),
            is_encrypted: true,
            patch_status: "Minor_Patch".to_string(),
            risk: RiskLevel::Low,
        });

        let mut threat_db = HashMap::new();
        threat_db.insert("203.0.113.42".to_string(), RiskLevel::Critical); // IP de C2 conhecido (Alto Risco)
        threat_db.insert("10.0.0.50".to_string(), RiskLevel::Low);       // IP Interno
        threat_db.insert("172.16.20.100".to_string(), RiskLevel::Medium); // IP de VPN de terceiros (Risco Médio)

        let mut time_p = HashMap::new();
        // Acesso restrito a Devs e Finanças fora do horário de 8h às 18h.
        time_p.insert("Developer".to_string(), (8, 18)); 
        time_p.insert("Finance".to_string(), (9, 17));

        PolicyEngine {
            user_database: user_db,
            device_database: device_db,
            threat_intel: threat_db,
            time_policies: time_p,
        }
    }

    /// Checagem de Política 1: Restrição baseada em horário de acesso.
    fn _check_time_policy(&self, user_role: &str, access_hour: u8) -> Result<(), String> {
        if let Some(&(start_h, end_h)) = self.time_policies.get(user_role) {
            if access_hour < start_h || access_hour >= end_h {
                return Err(format!(
                    "Violação de Política de Horário. Função '{}' só pode acessar entre {}:00h e {}:00h.", 
                    user_role, start_h, end_h
                ));
            }
        }
        Ok(())
    }

    /// Checagem de Política 2: Avaliação de Risco Comportamental.
    fn _check_behavioral_risk(&self, behavioral_score: u8) -> RiskLevel {
        if behavioral_score < 30 {
            RiskLevel::Critical
        } else if behavioral_score < 50 {
            RiskLevel::High
        } else {
            RiskLevel::Low
        }
    }

    /// Avalia o risco total com base no contexto (Dispositivo + IP + Comportamento).
    fn evaluate_total_risk(&self, device: &DevicePosture, source_ip: &str, behavioral_risk: RiskLevel) -> RiskLevel {
        let ip_risk = self.threat_intel.get(source_ip).copied().unwrap_or(RiskLevel::Medium);
        
        // O risco total é o maior nível encontrado entre os três vetores de risco.
        max(max(device.risk, ip_risk), behavioral_risk)
    }

    /// Implementa a lógica principal e complexa do "Never Trust, Always Verify".
    fn process_access_request(&self, request: &AccessRequest) -> AccessDecision {
        println!("\n--- Processando Solicitação: {} -> {} ---", request.user_id, request.resource);

        // 1. Identidade e Autenticação (Existe?)
        let user = match self.user_database.get(request.user_id) {
            Some(u) => u,
            None => return AccessDecision::Denied(format!("Identidade de usuário '{}' não encontrada.", request.user_id)),
        };

        // 2. Postura do Dispositivo (Verificação da "Saúde")
        let device = match self.device_database.get(request.device_id) {
            Some(d) => d,
            None => return AccessDecision::Denied(format!("Dispositivo '{}' não gerenciado ou desconhecido.", request.device_id)),
        };

        // 3. Acesso com Menos Privilégios (Least Privilege)
        if !user.allowed_resources.contains(request.resource) {
            return AccessDecision::Denied(format!("Violação de Least Privilege. Recurso '{}' não permitido para a função '{}'.", request.resource, user.role));
        }
        
        // 4. Checagem de Política de Horário
        if let Err(e) = self._check_time_policy(&user.role, request.access_hour) {
            return AccessDecision::Denied(e);
        }

        // 5. Avaliação de Risco Comportamental
        let behavioral_risk = self._check_behavioral_risk(user.behavioral_score);
        
        // 6. Avaliação de Risco Contextual Agregado (Postura + IP + Comportamento)
        let total_risk = self.evaluate_total_risk(device, request.source_ip, behavioral_risk);
        
        println!("   * Risco Comportamental: {} (Pontuação: {})", behavioral_risk, user.behavioral_score);
        println!("   * Risco Total Calculado: {} (Máximo permitido: {})", total_risk, user.max_risk);

        // 7. Decisão Final Baseada no Risco Total
        if total_risk > user.max_risk {
            return AccessDecision::Denied(format!("Risco total '{}' excede o limite permitido '{}'. Acesso negado.", total_risk, user.max_risk));
        }

        // 8. Permitir o Acesso e Gerar Token de Sessão
        let token = format!("{}-{}-{}-{}", request.user_id, request.resource, total_risk, request.access_hour);
        AccessDecision::Permitted { session_token: token, effective_risk: total_risk }
    }
}

// --- 3. Função Principal (Execução dos Cenários) ---

fn main() {
    println!("🦀 Iniciando Motor de Políticas Zero Trust Complexo (Rust)");
    let engine = PolicyEngine::new();

    // -------------------------------------------
    // CENÁRIO 1: ALICE - ACESSO PERMITIDO (Tudo OK)
    // Alice (Dev) acessando GIT_REPO, em horário de trabalho, de um dispositivo seguro e IP interno.
    // -------------------------------------------
    let request_alice_ok = AccessRequest {
        user_id: "alice", 
        device_id: "D_A_123", 
        source_ip: "10.0.0.50", 
        resource: "GIT_REPO", 
        access_hour: 10 // 10h da manhã
    };
    println!("Resultado CENÁRIO 1 (Alice OK): {:?}", engine.process_access_request(&request_alice_ok));
    
    // -------------------------------------------
    // CENÁRIO 2: CAROL - FALHA POR RISCO COMPORTAMENTAL
    // Carol (Financeiro) acessando FINANÇA, mas seu comportamento é suspeito (score 45).
    // O risco comportamental (High) excede o máximo permitido (Medium).
    // -------------------------------------------
    let request_carol_risk = AccessRequest {
        user_id: "carol", 
        device_id: "D_C_789", 
        source_ip: "10.0.0.50", 
        resource: "FINANCE_DB", 
        access_hour: 11 
    };
    println!("Resultado CENÁRIO 2 (Carol Risco Comportamental): {:?}", engine.process_access_request(&request_carol_risk));

    // -------------------------------------------
    // CENÁRIO 3: ALICE - FALHA POR VIOLAÇÃO DE HORÁRIO
    // Alice tenta acessar API_DEV fora do horário de política (23h).
    // -------------------------------------------
    let request_alice_time = AccessRequest {
        user_id: "alice", 
        device_id: "D_A_123", 
        source_ip: "10.0.0.50", 
        resource: "API_DEV_V1", 
        access_hour: 23 // 23h da noite
    };
    println!("Resultado CENÁRIO 3 (Alice Horário): {:?}", engine.process_access_request(&request_alice_time));

    // -------------------------------------------
    // CENÁRIO 4: BOB - FALHA POR POSTURA E IP SUSPEITO
    // Bob acessa CRM (permitido), mas usa um dispositivo desatualizado (Medium Risk) 
    // E de um IP de VPN de terceiros (Medium Risk).
    // O Risco Total (Medium) excede o máximo permitido para Bob (Low).
    // -------------------------------------------
    let request_bob_bad_context = AccessRequest {
        user_id: "bob", 
        device_id: "D_B_456", 
        source_ip: "172.16.20.100", 
        resource: "CRM_PROD", 
        access_hour: 14 
    };
    println!("Resultado CENÁRIO 4 (Bob Contexto Ruim): {:?}", engine.process_access_request(&request_bob_bad_context));
}
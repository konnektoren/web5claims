use aleo_sdk::*;
use web5claims::ZkProofClaim;
use yew::Callback;

pub struct AleoService {
    client: AleoClient,
    program_id: String,
}

impl AleoService {
    pub fn new() -> Self {
        Self {
            client: AleoClient::new("https://api.explorer.aleo.org/v1"),
            program_id: "web5claims.aleo".to_string(),
        }
    }

    pub async fn execute_issue_certificate(
        &self,
        certificate_data: &CertificateData,
        private_key: &PrivateKey,
    ) -> Result<Transaction, String> {
        // Convert certificate to Aleo inputs
        let inputs = certificate_to_aleo_inputs(certificate_data);

        // Execute the issue_certificate transition
        self.client
            .execute_program(&self.program_id, "issue_certificate", inputs, private_key)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn execute_prove_language_proficiency(
        &self,
        certificate_record: &Record,
        language: &str,
        min_level: u8,
        private_key: &PrivateKey,
    ) -> Result<ZkProofClaim, String> {
        let inputs = vec![
            language_to_field(language),
            format!("{}u8", min_level),
            format!("{}u32", chrono::Utc::now().timestamp()),
        ];

        let transaction = self
            .client
            .execute_program(
                &self.program_id,
                "prove_language_proficiency",
                inputs,
                private_key,
            )
            .await
            .map_err(|e| e.to_string())?;

        // Convert Aleo transaction to ZkProofClaim
        self.transaction_to_proof_claim(transaction)
    }
}

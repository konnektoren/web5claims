import web5claimsleo.aleo;

program test_web5claimsleo.aleo {

    @test
    function test_issue_certificate() {
        // Test issuing a certificate
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field; // German
        let level: u8 = 4u8; // B2
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );

        // Verify certificate properties
        assert_eq(certificate.owner, recipient);
        assert_eq(certificate.language, language);
        assert_eq(certificate.level, level);
        assert_eq(certificate.score, score);
    }

    @test
    function test_prove_language_proficiency() {
        // First create a certificate
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field; // German
        let level: u8 = 4u8; // B2
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );

        // Test proving language proficiency
        let min_level: u8 = 3u8; // B1 minimum
        let current_time: u32 = 1703664100u32;

        let proof: web5claimsleo.aleo/LanguageProficiencyProof = web5claimsleo.aleo/prove_language_proficiency(
            certificate,
            language,
            min_level,
            current_time
        );

        // Should meet requirement (B2 >= B1)
        assert_eq(proof.meets_requirement, true);
        assert_eq(proof.language, language);
        assert_eq(proof.min_level_required, min_level);
    }

    @test
    function test_prove_performance_threshold() {
        // Create a certificate
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 4u8;
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );

        // Test performance threshold proof
        let min_score: u8 = 90u8;
        let current_time: u32 = 1703664100u32;

        let proof: web5claimsleo.aleo/PerformanceProof = web5claimsleo.aleo/prove_performance_threshold(
            certificate,
            language,
            min_score,
            current_time
        );

        // Should meet threshold (94% >= 90%)
        assert_eq(proof.meets_threshold, true);
        assert_eq(proof.language, language);
        assert_eq(proof.min_score_required, min_score);
    }

    @test
    function test_prove_combined_requirements() {
        // Create a certificate
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 4u8; // B2
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );

        // Test combined proof
        let min_level: u8 = 3u8; // B1
        let min_score: u8 = 90u8;
        let current_time: u32 = 1703664100u32;

        let proof: web5claimsleo.aleo/CombinedProof = web5claimsleo.aleo/prove_combined_requirements(
            certificate,
            language,
            min_level,
            min_score,
            current_time
        );

        // Should meet both requirements
        assert_eq(proof.meets_language_requirement, true);
        assert_eq(proof.meets_performance_requirement, true);
        assert_eq(proof.language, language);
    }

    @test
    function test_prove_completion_rate() {
        // Create a certificate with known completion rate
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 4u8;
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16; // 94% completion rate
        let issued_at: u32 = 1703664000u32;

        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );

        // Test completion rate proof
        let min_completion: u8 = 90u8;
        let current_time: u32 = 1703664100u32;

        let proof: web5claimsleo.aleo/PerformanceProof = web5claimsleo.aleo/prove_completion_rate(
            certificate,
            language,
            min_completion,
            current_time
        );

        // Should meet completion threshold (94% >= 90%)
        assert_eq(proof.meets_threshold, true);
        assert_eq(proof.language, language);
    }

    @test
    @should_fail
    function test_invalid_level_should_fail() {
        // Test that invalid CEFR level fails
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 7u8; // Invalid level (> 6)
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        // This should fail due to assertion in issue_certificate
        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );
    }

    @test
    @should_fail
    function test_invalid_score_should_fail() {
        // Test that invalid score fails
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 4u8;
        let score: u8 = 101u8; // Invalid score (> 100)
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 47u16;
        let issued_at: u32 = 1703664000u32;

        // This should fail due to assertion in issue_certificate
        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );
    }

    @test
    @should_fail
    function test_invalid_challenges_should_fail() {
        // Test that invalid challenge count fails
        let recipient: address = aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc;
        let language: field = 1field;
        let level: u8 = 4u8;
        let score: u8 = 94u8;
        let challenges_total: u16 = 50u16;
        let challenges_solved: u16 = 51u16; // More solved than total
        let issued_at: u32 = 1703664000u32;

        // This should fail due to assertion in issue_certificate
        let certificate: web5claimsleo.aleo/LanguageCertificate = web5claimsleo.aleo/issue_certificate(
            recipient,
            language,
            level,
            score,
            challenges_total,
            challenges_solved,
            issued_at
        );
    }
}

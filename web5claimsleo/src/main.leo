program web5claimsleo.aleo {
    // Language Certificate Record
    // This is a private record that belongs to the certificate holder
    record LanguageCertificate {
        owner: address,
        language: field,           // Hash of language name (e.g., "german", "spanish")
        level: u8,                // CEFR level: 1=A1, 2=A2, 3=B1, 4=B2, 5=C1, 6=C2
        score: u8,                // Performance percentage (0-100)
        challenges_total: u16,    // Total number of challenges
        challenges_solved: u16,   // Number of challenges completed
        issued_at: u32,          // Timestamp when certificate was issued
        issuer: address,         // Address of the certificate issuer
    }

    // Public proof of language proficiency without revealing exact level/score
    struct LanguageProficiencyProof {
        language: field,
        meets_requirement: bool,
        min_level_required: u8,
        verified_at: u32,
    }

    // Public proof of performance threshold without revealing exact score
    struct PerformanceProof {
        language: field,
        meets_threshold: bool,
        min_score_required: u8,
        verified_at: u32,
    }

    // Combined proof showing both language level and performance
    struct CombinedProof {
        language: field,
        meets_language_requirement: bool,
        meets_performance_requirement: bool,
        min_level_required: u8,
        min_score_required: u8,
        verified_at: u32,
    }

    // Issue a new language learning certificate
    // This transition creates a private record for the certificate holder
    transition issue_certificate(
        public recipient: address,
        public language: field,
        public level: u8,
        public score: u8,
        public challenges_total: u16,
        public challenges_solved: u16,
        public issued_at: u32
    ) -> LanguageCertificate {
        // Validate inputs
        assert(level >= 1u8 && level <= 6u8);  // Valid CEFR levels
        assert(score <= 100u8);                 // Valid percentage
        assert(challenges_solved <= challenges_total); // Can't solve more than total

        return LanguageCertificate {
            owner: recipient,
            language: language,
            level: level,
            score: score,
            challenges_total: challenges_total,
            challenges_solved: challenges_solved,
            issued_at: issued_at,
            issuer: self.caller,
        };
    }

    // Prove language proficiency level without revealing exact level or score
    // Returns true if the certificate holder has achieved at least the minimum level
    transition prove_language_proficiency(
        private certificate: LanguageCertificate,
        public language_required: field,
        public min_level: u8,
        public current_time: u32
    ) -> LanguageProficiencyProof {
        // Verify certificate ownership
        assert_eq(certificate.owner, self.caller);

        // Verify this is for the correct language
        assert_eq(certificate.language, language_required);

        // Check if the certificate meets the minimum level requirement
        let meets_requirement: bool = certificate.level >= min_level;

        return LanguageProficiencyProof {
            language: language_required,
            meets_requirement: meets_requirement,
            min_level_required: min_level,
            verified_at: current_time,
        };
    }

    // Prove performance threshold without revealing exact score
    // Returns true if the certificate holder achieved at least the minimum score
    transition prove_performance_threshold(
        private certificate: LanguageCertificate,
        public language_required: field,
        public min_score: u8,
        public current_time: u32
    ) -> PerformanceProof {
        // Verify certificate ownership
        assert_eq(certificate.owner, self.caller);

        // Verify this is for the correct language
        assert_eq(certificate.language, language_required);

        // Check if the certificate meets the minimum score requirement
        let meets_threshold: bool = certificate.score >= min_score;

        return PerformanceProof {
            language: language_required,
            meets_threshold: meets_threshold,
            min_score_required: min_score,
            verified_at: current_time,
        };
    }

    // Prove both language level and performance in a single proof
    // This is more efficient than generating two separate proofs
    transition prove_combined_requirements(
        private certificate: LanguageCertificate,
        public language_required: field,
        public min_level: u8,
        public min_score: u8,
        public current_time: u32
    ) -> CombinedProof {
        // Verify certificate ownership
        assert_eq(certificate.owner, self.caller);

        // Verify this is for the correct language
        assert_eq(certificate.language, language_required);

        // Check both requirements
        let meets_language: bool = certificate.level >= min_level;
        let meets_performance: bool = certificate.score >= min_score;

        return CombinedProof {
            language: language_required,
            meets_language_requirement: meets_language,
            meets_performance_requirement: meets_performance,
            min_level_required: min_level,
            min_score_required: min_score,
            verified_at: current_time,
        };
    }

    // Prove completion rate without revealing exact numbers
    // Useful for showing dedication/completion percentage thresholds
    transition prove_completion_rate(
        private certificate: LanguageCertificate,
        public language_required: field,
        public min_completion_percentage: u8,
        public current_time: u32
    ) -> PerformanceProof {
        // Verify certificate ownership
        assert_eq(certificate.owner, self.caller);

        // Verify this is for the correct language
        assert_eq(certificate.language, language_required);

        // Calculate completion percentage
        let completion_rate: u16 = (certificate.challenges_solved * 100u16) / certificate.challenges_total;
        let completion_percentage: u8 = completion_rate as u8;

        // Check if meets minimum completion rate
        let meets_threshold: bool = completion_percentage >= min_completion_percentage;

        return PerformanceProof {
            language: language_required,
            meets_threshold: meets_threshold,
            min_score_required: min_completion_percentage,
            verified_at: current_time,
        };
    }

    // Prove certificate recency (issued within a certain timeframe)
    // Useful for proving recent language study
    transition prove_certificate_recency(
        private certificate: LanguageCertificate,
        public language_required: field,
        public min_issued_time: u32,
        public current_time: u32
    ) -> LanguageProficiencyProof {
        // Verify certificate ownership
        assert_eq(certificate.owner, self.caller);

        // Verify this is for the correct language
        assert_eq(certificate.language, language_required);

        // Check if certificate was issued after the minimum time
        let is_recent: bool = certificate.issued_at >= min_issued_time;

        return LanguageProficiencyProof {
            language: language_required,
            meets_requirement: is_recent,
            min_level_required: 0u8, // Not checking level here
            verified_at: current_time,
        };
    }
}

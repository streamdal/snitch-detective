#[cfg(test)]
use protos::matcher::{MatchRequest, MatchType};

#[test]
fn test_numeric() {
    let test_cases = vec![
        // Equal
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO,
                &"number_int".to_string(),
                vec!["100".to_string()],
                false,
            ),
            expected: true,
            text: "equal number_int".to_string(),
            should_error: false,
        },
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO,
                &"number_float".to_string(),
                vec!["100.1".to_string()],
                false,
            ),
            expected: true,
            text: "equal number_float".to_string(),
            should_error: false,
        },
        // Greater than
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN,
                &"number_int".to_string(),
                vec!["1".to_string()],
                false,
            ),
            expected: true,
            text: "greater than number_int".to_string(),
            should_error: false,
        },
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN,
                &"number_float".to_string(),
                vec!["2".to_string()],
                false,
            ),
            expected: true,
            text: "greater than number_float".to_string(),
            should_error: false,
        },
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_GREATER_THAN,
                &"number_float".to_string(),
                vec!["1000".to_string()],
                false,
            ),
            expected: false,
            text: "NOT greater than number_float".to_string(),
            should_error: false,
        },
        // Greater equal
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_GREATER_EQUAL,
                &"number_float".to_string(),
                vec!["100.1".to_string()],
                false,
            ),
            expected: true,
            text: "greater or equal than number_float".to_string(),
            should_error: false,
        },
        // Less than
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_LESS_THAN,
                &"number_int".to_string(),
                vec!["2000".to_string()],
                false,
            ),
            expected: true,
            text: "less than number_int".to_string(),
            should_error: false,
        },
        // Less equal
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL,
                &"number_int".to_string(),
                vec!["1000".to_string()],
                false,
            ),
            expected: true,
            text: "less equal than number_int 1".to_string(),
            should_error: false,
        },
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL,
                &"number_int".to_string(),
                vec!["999".to_string()],
                false,
            ),
            expected: true,
            text: "less equal than number_int 2".to_string(),
            should_error: false,
        },
        // Negate
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_LESS_EQUAL,
                &"number_int".to_string(),
                vec!["99".to_string()],
                false,
            ),
            expected: false,
            text: "Negate: less equal than number_int".to_string(),
            should_error: false,
        },
        // Error paths
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO,
                &"number_int".to_string(),
                vec!["not a number".to_string()],
                false,
            ),
            should_error: true,
            expected: false,
            text: "equal number_int bad arg".to_string(),
        },
        crate::test_utils::TestCase {
            request: crate::test_utils::generate_request(
                MatchType::MATCH_TYPE_NUMERIC_EQUAL_TO,
                &"does_not_exist".to_string(),
                vec!["1000".to_string()],
                false,
            ),
            should_error: true,
            expected: true,
            text: "equal number_int bad path".to_string(),
        },
    ];

    crate::test_utils::run_tests(&test_cases);
}
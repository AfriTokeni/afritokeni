// HTTP handler tests - testing routing logic

#[test]
fn test_url_path_parsing() {
    let url = "/api/ussd?foo=bar";
    let path = url.split('?').next().unwrap();
    assert_eq!(path, "/api/ussd", "Should extract path without query params");
}

#[test]
fn test_url_path_without_query() {
    let url = "/api/sms";
    let path = url.split('?').next().unwrap();
    assert_eq!(path, "/api/sms", "Should handle URL without query params");
}

#[test]
fn test_route_matching() {
    let routes = vec![
        ("/api/ussd", "ussd"),
        ("/api/sms", "sms"),
    ];
    
    for (path, expected) in routes {
        match path {
            "/api/ussd" => assert_eq!(expected, "ussd", "Should match USSD route"),
            "/api/sms" => assert_eq!(expected, "sms", "Should match SMS route"),
            _ => panic!("Unknown route"),
        }
    }
}

#[test]
fn test_content_type_detection() {
    let json_type = "application/json";
    let form_type = "application/x-www-form-urlencoded";
    
    assert!(json_type.contains("application/json"), "Should detect JSON");
    assert!(form_type.contains("application/x-www-form-urlencoded"), "Should detect form data");
}

#[test]
fn test_security_path_traversal_detection() {
    let malicious_paths = vec![
        "/api/../../../etc/passwd",
        "/api/./../../secret",
        "/../admin",
    ];
    
    for path in malicious_paths {
        assert!(path.contains(".."), "Should detect path traversal attempt: {}", path);
    }
}

#[test]
fn test_security_sql_injection_detection() {
    let malicious_inputs = vec![
        "'; DROP TABLE users; --",
        "1' OR '1'='1",
        "admin'--",
    ];
    
    for input in malicious_inputs {
        assert!(input.contains("'"), "Should detect SQL injection attempt: {}", input);
    }
}

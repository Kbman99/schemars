mod util;
use ipnet::IpNet;
use util::*;

#[test]
fn ipnet() -> TestResult {
    test_default_generated_schema::<IpNet>("ipnet")
}

use std::collections::HashMap;

use crate::common::Value;
use crate::source::{BooleanSource, NumberSource, SourcePrimitive};

fn expect_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) {
    assert!(std::panic::catch_unwind(f).is_err());
}

#[test]
fn number_source_requires_parameter() {
    let source = NumberSource::new();
    let outputs = source.produce(&HashMap::from([(
        "value".to_string(),
        crate::source::ParameterValue::Number(3.5),
    )]));
    assert_eq!(outputs.get("value"), Some(&Value::Number(3.5)));

    expect_panic(|| {
        source.produce(&HashMap::new());
    });
}

#[test]
fn boolean_source_requires_parameter() {
    let source = BooleanSource::new();
    let outputs = source.produce(&HashMap::from([(
        "value".to_string(),
        crate::source::ParameterValue::Bool(true),
    )]));
    assert_eq!(outputs.get("value"), Some(&Value::Bool(true)));

    expect_panic(|| {
        source.produce(&HashMap::new());
    });
}

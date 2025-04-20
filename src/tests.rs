#[cfg(test)]
use crate::Frequency;

#[test]
fn test_parse_valid_units() {
    assert_eq!("1hz".parse::<Frequency>().unwrap(), Frequency::from_hz(1));
    assert_eq!(
        "2.5khz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(2500)
    );
    assert_eq!(
        "1.5mhz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(1_500_000)
    );
    assert_eq!(
        "0.1ghz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(100_000_000)
    );

    assert_eq!("1 Hz".parse::<Frequency>().unwrap(), Frequency::from_hz(1));
    assert_eq!(
        "2.5 kHz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(2500)
    );
    assert_eq!(
        "1.5 MHz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(1_500_000)
    );
    assert_eq!(
        "0.1 GHz".parse::<Frequency>().unwrap(),
        Frequency::from_hz(100_000_000)
    );
}

#[test]
fn test_parse_invalid_values() {
    assert!("abc".parse::<Frequency>().is_err());
    assert!("42".parse::<Frequency>().is_err()); // no unit
    assert!("20h".parse::<Frequency>().is_err()); // invalid unit
    assert!("2.5mh".parse::<Frequency>().is_err()); // incomplete unit
    assert!("GHz".parse::<Frequency>().is_err()); // no value
    assert!("".parse::<Frequency>().is_err()); // empty
    assert!("--5GHz".parse::<Frequency>().is_err()); // invalid number
}

#[test]
fn test_roundtrip_display_parse() {
    let freqs = [
        Frequency::from_hz(1340),
        Frequency::from_khz(42),
        Frequency::from_mhz(100),
        Frequency::from_ghz(2),
        Frequency::from_ghz(3) + Frequency::from_mhz(250),
    ];

    for f in freqs {
        let display = f.to_string();
        let parsed = display.parse::<Frequency>().unwrap();
        assert_eq!(parsed, f, "roundtrip failed for: {display}");
    }
}

#[test]
fn test_unit_conversions() {
    let freq = Frequency::from_ghz(1);
    assert_eq!(freq.as_hz(), 1_000_000_000);
    assert_eq!(freq.as_khz(), 1_000_000);
    assert_eq!(freq.as_mhz(), 1_000);
    assert_eq!(freq.as_ghz(), 1);

    let freq = Frequency::from_mhz(1500);
    assert_eq!(freq.as_hz(), 1_500_000_000);
    assert_eq!(freq.as_khz(), 1_500_000);
    assert_eq!(freq.as_ghz(), 1); // truncation
}

#[test]
fn test_add_and_sub() {
    let f1 = Frequency::from_mhz(1000);
    let f2 = Frequency::from_mhz(500);

    assert_eq!(f1 + f2, Frequency::from_mhz(1500));
    assert_eq!(f1 - f2, Frequency::from_mhz(500));
}

#[test]
fn test_as_duration() {
    let freq = Frequency::from_ghz(1);
    let duration = freq.as_duration();
    assert_eq!(duration.as_nanos(), 1);

    let freq = Frequency::from_mhz(1);
    let duration = freq.as_duration();
    assert_eq!(duration.as_nanos(), 1_000);

    let freq = Frequency::from_hz(0);
    let duration = freq.as_duration();
    assert_eq!(duration.as_nanos(), 0);
}

#[test]
fn test_display_formatting() {
    assert_eq!(Frequency::from_hz(42).to_string(), "42 Hz");
    assert_eq!(Frequency::from_khz(100).to_string(), "100.00 kHz");
    assert_eq!(Frequency::from_mhz(3).to_string(), "3.00 MHz");
    assert_eq!(Frequency::from_ghz(2).to_string(), "2.00 GHz");
    assert_eq!(
        (Frequency::from_ghz(2) + Frequency::from_mhz(250)).to_string(),
        "2.25 GHz"
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    use serde_json;

    let freq = Frequency::from_mhz(1340);
    let json = serde_json::to_string(&freq).unwrap();
    assert_eq!(json, "\"1.34 GHz\"");

    let parsed: Frequency = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, freq);
}

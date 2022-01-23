use std::fs::read_to_string;
use std::fs::write;

#[allow(dead_code)]
fn test1() -> Result<(), std::io::Error> {
    let file_name: String = "src/data.json".to_string();
    let contents = read_to_string(file_name)?;
    let contents = contents.replace("\n", "");
    let parsed = json::parse(&contents);
    println!("{:?}", contents);
    println!("{:?}", parsed);
    write("src/out2.json", contents)?;
    Ok(())
}

#[test]
fn deserialize() {
    let parsed = json::parse(
        r#"
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}
"#,
    )
    .unwrap();
    use json::object;
    let instantiated = object! {
        // quotes on keys are optional
        "code": 200,
        success: true,
        payload: {
            features: [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };
    println!("{:?}", parsed);
    println!("{:?}", instantiated);
    assert_eq!(parsed, instantiated);
}

fn test2() {
    use json::object;
    let mut data = object! {
        foo: false,
        bar: null,
        answer: 42,
        list: [null, "world", true]
    };

    // Partial equality is implemented for most raw types:
    assert!(data["foo"] == false);

    // And it's type aware, `null` and `false` are different values:
    assert!(data["bar"] != false);

    // But you can use any Rust number types:
    assert!(data["answer"] == 42);
    assert!(data["answer"] == 42.0);
    assert!(data["answer"] == 42isize);

    // Access nested structures, arrays and objects:
    assert!(data["list"][0].is_null());
    assert!(data["list"][1] == "world");
    assert!(data["list"][2] == true);

    // Error resilient - accessing properties that don't exist yield null:
    assert!(data["this"]["does"]["not"]["exist"].is_null());

    // Mutate by assigning:
    data["list"][0] = "Hello".into();

    // Use the `dump` method to serialize the data:
    assert_eq!(
        data.dump(),
        r#"{"foo":false,"bar":null,"answer":42,"list":["Hello","world",true]}"#
    );

    // Or pretty print it out:
    println!("{:#}", data);
    println!("OK");
}

fn serialize() {
    // str slices
    assert_eq!(json::stringify("foobar"), "\"foobar\"");

    // Owned strings
    assert_eq!(json::stringify("foobar".to_string()), "\"foobar\"");

    // Any number types
    assert_eq!(json::stringify(42), "42");

    // Booleans
    assert_eq!(json::stringify(true), "true");
    assert_eq!(json::stringify(false), "false");
}

fn main() -> Result<(), std::io::Error> {
    // test1()?;
    // deserialize();
    test2();
    serialize();
    Ok(())
}

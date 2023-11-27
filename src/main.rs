use json_parser::parse;

fn main() {
    let parsed = parse(
        r#"{"code": 200,
            "success": true,
            "notrust": null,
            "payload": {
                "features": [
                    "recursive",
                    "easy",
                    "fun"
                ]
            }
        }"#,
    );

    println!("{:#?}", parsed);
}

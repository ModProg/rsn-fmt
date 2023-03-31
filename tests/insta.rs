use include_dir::include_dir;

const INPUTS: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/tests/input");

#[test]
fn format() {
    for file in INPUTS.entries() {
        let contents = file.as_file().unwrap().contents_utf8().unwrap();
        // TODO let before: OwnedValue = rsn::from_str(contents).unwrap();

        let formatted = rsn_fmt::format_str(&contents);

        println!("{formatted}");
        // let after: OwnedValue = rsn::from_str(formatted).unwrap();
        // assert_eq!(before, after);

        insta::assert_snapshot!(file.path().display().to_string(), formatted);
    }
}

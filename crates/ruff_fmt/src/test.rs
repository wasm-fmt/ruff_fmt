#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::PathBuf};
    use testing_macros::fixture;

    use crate::format;

    #[fixture("test_data/*.input")]
    fn it_works(input: PathBuf) {
        // calc the expected file path
        let extect_path = input.with_extension("expect");

        let mut actual = String::new();
        File::open(&input).and_then(|mut file| file.read_to_string(&mut actual)).unwrap();

        let mut expect = String::new();
        File::open(extect_path).and_then(|mut file| file.read_to_string(&mut expect)).unwrap();

        let actual = format(&actual).unwrap();

        assert_eq!(actual, expect);
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::PathBuf, str::FromStr};
    use testing_macros::fixture;

    use crate::format;

    #[fixture("test_data/**/*.py")]
    #[fixture("test_data/**/*.pyi")]
    fn it_works(input: PathBuf) {
        // calc the expected file path
        let input = input.clone();
        let extect_path = input.to_string_lossy() + ".expect";
        let extect_path = PathBuf::from_str(&extect_path).unwrap();

        let mut actual = String::new();
        File::open(&input).and_then(|mut file| file.read_to_string(&mut actual)).unwrap();

        let mut expect = String::new();
        File::open(extect_path).and_then(|mut file| file.read_to_string(&mut expect)).unwrap();

        let actual = format(&actual, Some(input.to_string_lossy().to_string()), None).unwrap();

        assert_eq!(actual, expect);
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::format;

    #[test]
    fn ruff_format() {
        insta::with_settings!({
            snapshot_path => "../snapshots",
        }, {
            insta::glob!("../test_data", "**/*.{py,pyi}", |path| {
                let mut input = String::new();
                File::open(path)
                    .and_then(|mut file| file.read_to_string(&mut input))
                    .unwrap();

                let output = format(&input, Some(path.to_string_lossy().to_string()), None).unwrap();
                let ext = match path.extension().and_then(|e| e.to_str()) {
                    Some("py") => ".py",
                    Some("pyi") => ".pyi",
                    _ => ".txt",
                };
                insta::assert_binary_snapshot!(ext, output.into_bytes());
            });
        });
    }
}

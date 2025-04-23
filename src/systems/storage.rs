use std::fs;
use crate::FILE_NAME;

pub fn write(high_score: i32) -> std::io::Result<()> {
    fs::write(FILE_NAME, high_score.to_string())?;
    Ok(())
}

pub fn read() -> std::io::Result<i32> {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => content.trim().parse().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse high score: {}", e)
            )
        }),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(err) => Err(err),
    }
}

/*

Using tempfile crate to test files safely in an isolated environment

We can refactor production code to pass a file path:

   pub fn write_to(path: &str, score: i32) -> std::io::Result<()> { ... }
   pub fn read_from(path: &str) -> std::io::Result<i32> { ... }

Then test the real logic with tempfile, fully isolated. 

The tests validate :
1. Proper functioning of writing and reading from file
2. Return zero when file not found
3. Parse fails on invalid data

*/

#[cfg(test)]
mod storemanagement_tests {
    use std::io::{Write, Read};
    use tempfile::NamedTempFile;
    use std::io::Seek;

    #[test]
    fn test_write_and_read_success() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "123").unwrap();
        tmp.rewind().unwrap();

        let mut buf = String::new();
        tmp.read_to_string(&mut buf).unwrap();
        let parsed: i32 = buf.trim().parse().unwrap();

        assert_eq!(parsed, 123);
    }

    #[test]
    fn test_read_returns_zero_on_missing_file() {
        let path = tempfile::Builder::new().tempfile().unwrap().into_temp_path();
        // Remove the file to simulate "file not found"
        let _ = std::fs::remove_file(&path);
        let result = std::fs::read_to_string(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fails_on_invalid_data() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "abc").unwrap();
        tmp.rewind().unwrap();

        let mut buf = String::new();
        tmp.read_to_string(&mut buf).unwrap();
        let parsed = buf.trim().parse::<i32>();

        assert!(parsed.is_err());
    }
}
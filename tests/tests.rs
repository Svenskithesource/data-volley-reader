#[cfg(test)]
mod tests {
    use data_volley_reader::read;

    #[test]
    fn test_file() {
        let file = std::fs::File::open("tests/test.dvw").unwrap();
        let mut buffer = std::io::BufReader::new(file);
        let scout_file = read(&mut buffer).unwrap();
        dbg!(scout_file);
    }
}

/// ctv images/ --format webp --quality 100
#[derive(Debug)]
pub struct Command {
    paths: Vec<String>,
    format: Option<String>,
    quality: Option<u8>,
    recursive: bool,
}

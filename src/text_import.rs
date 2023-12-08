

pub fn read_pdf(path: &str) -> String {
    let bytes = std::fs::read(path).unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    out
}
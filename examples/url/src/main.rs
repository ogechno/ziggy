fn main() {
    ziggy::fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            #[allow(unused_variables)]
            if let Ok(parsed) = url::Url::parse(s) {
                #[cfg(not(fuzzing))]
                println!("{parsed}");
            }
        }
    });
}

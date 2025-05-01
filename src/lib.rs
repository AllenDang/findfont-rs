pub mod findfont;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let font_name = "STHeiti";
        let font = findfont::find(font_name);

        assert!(font.is_some());
    }
}

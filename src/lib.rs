mod findfont;

pub use findfont::find;
pub use findfont::get_sys_font_dir;
pub use findfont::get_sys_font_dirs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_works() {
        let font_name = "Arial";
        let font = findfont::find(font_name);

        println!("Font path: {}", font.clone().unwrap().display());
        assert!(font.is_some());
    }

	#[test]
	fn get_sys_font_dir_works() {
		let dir = findfont::get_sys_font_dir();

		println!("Font path: {}", dir.clone().unwrap().display());
		assert!(dir.is_some());
	}
}

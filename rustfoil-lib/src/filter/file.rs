use crate::abstraction::file::TinfoilFileLike;
use lazy_static::lazy_static;
use regex::Regex;

pub static NSW_EXTENSIONS: [&str; 4] = [".nsp", ".nsz", ".xci", ".xcz"];

pub fn filter_files<T>(
    files: Vec<T>,
    add_non_nsw_files: bool,
    add_nsw_files_without_title_id: bool,
) -> Vec<T>
where
    T: TinfoilFileLike,
{
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[0-9A-Fa-f]{16}").unwrap();
    }

    let mut result = Vec::new();

    for file in files {
        let mut keep = true;
        let file_name = file.get_name();

        if !add_non_nsw_files {
            let extension: String = file_name
                .chars()
                .skip(file_name.len() - 4)
                .take(4)
                .collect();

            keep = NSW_EXTENSIONS.contains(&extension.as_str());
        }

        if !add_nsw_files_without_title_id {
            keep = REGEX.is_match(file_name);
        }

        if keep {
            result.push(file);
        }
    }

    return result;
}

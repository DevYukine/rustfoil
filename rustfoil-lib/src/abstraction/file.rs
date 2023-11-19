pub trait TinfoilFileLike {
    fn get_url(&self) -> String;
    fn get_size(&self) -> i64;
    fn get_name(&self) -> &str;
}

//todo: make it only for Error impl
pub trait GitInfo {
    fn get_git_info(&self) -> String; //return lazy static String (execute one time on program start)
}

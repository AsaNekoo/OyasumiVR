pub fn is_elevated() -> bool {
        //on linux its possible user is able to preform privilaged action without being root
        //needs to be checked per action
        false 
}

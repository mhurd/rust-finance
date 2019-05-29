fn main() {
    use leg::*;
    use std::env;

    let args: Vec<String> = env::args().collect();
    let args_string = format!("{:?}", args);
    let scope = "Program args";

    info(&args_string, Some(&scope), None);
    success(&args_string, Some(&scope), None);
    warn(&args_string, Some(&scope), None);
    error(&args_string, Some(&scope), None);
    wait(&args_string, Some(&scope), None);
    done(&args_string, Some(&scope), None);

}

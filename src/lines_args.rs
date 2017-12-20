use std::path::Path;

#[derive(Deserialize)]
pub struct Args {
    pub arg_file: String,
    pub arg_start: u64,
    pub arg_stop: u64,
    pub flag_lines: bool,
    pub flag_version: bool,
    pub flag_spaces: Option<usize>,
}

/**
 * Checks to see if the arguments provided are valid or not. A valid argument is one where:
 *
 * 1. The file exists
 * 2. The start value is less than or equal to the stop value
 *
 * # Arguments
 * - `args` - The `Args` structure in question
 *
 * # Returns
 * `Ok()` if `args` is valid, `Err(e)` otherwise.
 **/
pub fn check_args(args: Args) -> Result<Args, &'static str> {
    if !Path::new(&args.arg_file).exists() {
        Err("File does not exist")
    } else if args.arg_start > args.arg_stop {
        Err("Start value is greater than stop value")
    } else {
        Ok(args)
    }
}

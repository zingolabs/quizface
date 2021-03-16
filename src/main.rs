use quizface::{
    check_success, get_command_help, ingest_commands, produce_interpretation,
    utils::logging::create_version_name, utils::logging::log_raw_output,
};
fn process_command(command: &str) {
    let command_help_output = get_command_help(command);

    check_success(&command_help_output.status);

    let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
        .expect("Invalid raw_command_help, error!");

    log_raw_output(command, raw_command_help.to_string());
    //select just for blessed results.
    produce_interpretation(raw_command_help);
    dbg!(command);
}
fn main() {
    let args = &std::env::args().collect::<Vec<String>>()[1..];
    if args.len() == 0 {
        for command in ingest_commands() {
            process_command(&command);
        }
    } else {
        for command in args {
            process_command(&command);
        }
    };
    println!("main() complete!");
}

use quizface::{
    get_command_help, produce_interpretation,
    utils::logging::{create_log_dirs, log_raw_output},
};
fn process_command(command: &str) {
    let command_help_output = get_command_help(command);
    if !&command_help_output.status.success() {
        panic!()
    };

    let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
        .expect("Invalid raw_command_help, error!");

    log_raw_output(command, raw_command_help.to_string());
    //select just for blessed results.
    produce_interpretation(raw_command_help);
}
fn main() {
    create_log_dirs();
    for command in &std::env::args().collect::<Vec<String>>()[1..] {
        process_command(&command);
    }
    dbg!("SUCCESS!");
}

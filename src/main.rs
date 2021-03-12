fn main() {
    use quizface::{
        check_success, get_command_help, ingest_commands,
        produce_interpretation, utils::logging::log_raw_output,
    };
    for command in ingest_commands() {
        let command_help_output = get_command_help(&command);
        check_success(&command_help_output.status);
        let raw_command_help = std::str::from_utf8(&command_help_output.stdout)
            .expect("Invalid raw_command_help, error!");
        log_raw_output(&command, raw_command_help.to_string());
        //select just for blessed results.
<<<<<<< Updated upstream
        if blessed_tome.contains(&command) {
            //dbg!(&command);
            produce_interpretation(raw_command_help);
        }
=======
        dbg!(&command);
        produce_interpretation(raw_command_help);
>>>>>>> Stashed changes
    }
    println!("main() complete!");
}

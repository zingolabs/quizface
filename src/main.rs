fn main() {
    use quizface::{
        check_success, get_command_help, ingest_commands,
        produce_interpretation, utils::logging::create_version_name,
        utils::logging::log_raw_output, utils::prescrubbing::prescrub,
    };
    use std::io::BufRead;

    let commands = ingest_commands();

    // make blessed and mundane tomes
    // assumes blessed --bin has previously been run
    // or blessed.txt has checked in
    // and mundane.txt is checked in
    let blessed_location = format!(
        "./logs/{}/blessed_commands/blessed.txt",
        create_version_name()
    );
    let blessed_path = std::path::Path::new(&blessed_location);
    let blessed_reader = std::io::BufReader::new(
        std::fs::File::open(blessed_path).expect("Alas! No blessed text!"),
    );
    let blessed_tome: Vec<String> = blessed_reader
        .lines()
        .map(|l| l.expect("Could not make line."))
        .collect();

    let mundane_location = "./lists/mundate.txt";
    let mundane_path = std::path::Path::new(&mundane_location);
    let mundane_reader = std::io::BufReader::new(
        std::fs::File::open(mundane_path).expect("No mundane text!"),
    );
    let mundane_tome: Vec<String> = mundane_reader
        .lines()
        .map(|l| l.expect("Could not make line."))
        .collect();

    for command in commands {
        let command_help_output = get_command_help(&command);
        check_success(&command_help_output.status);
        let raw_command_help = std::str::from_utf8(&command_help_output.stdout).expect("Invalid raw_command_help, error!");
        log_raw_output(&command, raw_command_help.to_string());
        //raw_command_help is &str

        //select just for blessed and mundane results.
        if blessed_tome.contains(&command) {
            //dbg!(&command);
            produce_interpretation(raw_command_help);
        } else if mundane_tome.contains(&command) {
            let prescrubbed_mundane_help: String = prescrub(&command, raw_command_help);
            produce_interpretation(prescrubbed_mundane_help.as_str());
        } else {
            // cursed
        }
    }
    println!("main() complete!");
}

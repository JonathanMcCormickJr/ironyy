use std::io;

/// Gets user input from the command line.
pub fn get_user_input() -> Result<String, io::Error> {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)?;

    Ok((user_input).trim().to_string())
}

/// Waits for the user to press Enter.
pub fn wait_for_key_press() -> Result<(), io::Error> {
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}
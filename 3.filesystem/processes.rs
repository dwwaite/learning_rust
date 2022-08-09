use std::process::{Command,Stdio};

fn main() {
    // There is a basic struct for executing tools available on the PATH.
    let status = Command::new("rustc")
        .arg("-V")
        .status()
        .expect("no rustc?");

    println!("Results of status:");
    println!("  {}", status.code().unwrap());
    println!("  {}", status.success());

    /* That's handy - but it leaks the stdout and stderr back to the terminal.
     *    To capture them, slightly different invocation.
     * Unfortunately (but not unusually), the results come back as a Vec<u8>.
     */
    let status_with_std = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("no rustc?");

    println!("\nResults of captured status:");
    println!("  stdout (raw): `{:?}`", status_with_std.stdout);

    // There is also the `from_utf8()` function, but this is better for capturing non-standard output.
    let stdout_str = String::from_utf8_lossy(&status_with_std.stdout).trim_end().to_string();
    println!("  stdout (str): `{}`", stdout_str);

    /* Currently, these are executing as blocking processes. What is we want to let it
     *    run in the background and wait?
     * Will also use this as an example of redicting stdout/stderr into `/dev/null` in a
     *    platform-agnostic way.
     */
    let mut status_from_bg = Command::new("rustc")
        .arg("-V")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("no rustc?");

    let output = status_from_bg.wait();
    println!("\nResults of background status:");
    println!("  `{:?}`", output);

    /* Finally, and speaking of platform-agnostic redirection, what if we want to write the code
     *    so that we don't need to change details when compiling on different operating systems?
     * We have the configuration conditional checks (`cfg!`) macro for this.
     */
    let shell_name = if cfg!(windows) {"cmd.exe"} else {"/usr/bin/sh"};
    let command_name = if cfg!(windows) {"dir"} else {"ls"};
    let flag_value = if cfg!(windows) {"/s"} else {"-sh"};

    let status_with_cfg = Command::new(shell_name)
        .arg(command_name)
        .arg(flag_value)
        .output()
        .expect("no shell?");
    
    let stderr_str = String::from_utf8_lossy(&status_with_cfg.stderr).trim_end().to_string();
    let stdout_str = String::from_utf8_lossy(&status_with_cfg.stdout).trim_end().to_string();
    println!("\nResults of cfg! status:");
    println!("  success: {}", status_with_cfg.status.success());
    println!("  stderr: `{}`", stderr_str);
    println!("  stdout: `{}`", stdout_str);
    
}
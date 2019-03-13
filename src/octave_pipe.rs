use std::process::*;

fn submit_octave(code: String) -> (String, String) {
    let process = match Command::new("/usr/bin/octave").arg("-q").arg("--eval")
                                .arg(format!("{}", code))
                                .stdout(Stdio::piped()).stderr(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn octave: {}", why),
        Ok(process) => process,
    };
    let mut out = String::new();
    process.stdout.unwrap().read_to_string(&mut out).expect("read octave stdout pipe failed");
    let mut err = String::new();
    process.stderr.unwrap().read_to_string(&mut err).expect("read octave stderr pipe failed");
    (out, err)
}

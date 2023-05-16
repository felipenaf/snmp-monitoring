use std::process::Command;

pub fn ls() {
    let ls = Command::new("ls")
        .output()
        .expect("failed to execute ls");

    let stdout = String::from_utf8_lossy(&ls.stdout);
    // let lines: Vec<&str> = stdout.lines().map(String::from).collect();
    let lines: Vec<&str> = stdout.lines().collect();
    println!("{:?}", lines);

    // for s in lines {
    //     println!("{}", s);
    // }
}

pub fn snmpget() {
    let snmpget = Command::new("snmpget")
        .arg("-v2c")
        .arg("-c")
        .arg("public")
        .arg("10.60.0.40")
        .arg("SNMPv2-SMI::enterprises.13727.2300.2.101.10.10.10.4.0")
        .output()
        .expect("failed to execute snmpget command");

    println!("{}", String::from_utf8_lossy(&snmpget.stdout));
}

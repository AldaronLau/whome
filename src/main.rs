// Who Me?
// Copyright © 2017-2025 Jeryn Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use whoami::Result;
use yansi::Paint;

fn version() {
    println!(
        "{}{} {}",
        "aldaronlau.".bright_red(),
        env!("CARGO_PKG_NAME").bright_blue().bold(),
        env!("CARGO_PKG_VERSION").bright_green(),
    );
    println!(
        "{} {}",
        "Copyright ©".bold(),
        "Jeryn Lau 2017 - 2025.".magenta(),
    );
    println!(
        "{} {}",
        "License".bold(),
        "Apache-2.0 OR BSL-1.0 OR MIT".magenta(),
    );
}

fn help() {
    println!(
        "{} {} {}",
        "Usage".bold(),
        "whome".bright_green(),
        "[OPTION]".bright_cyan(),
    );
    println!("Print the name of the user who is logged in.\n");
    println!(
        "{}Print this help and exit.",
        "    help            ".bright_cyan(),
    );
    println!(
        "{}Print version and exit.",
        "    version         ".bright_cyan(),
    );
    println!(
        "{}Print the user's full name.",
        "    realname        ".bright_cyan(),
    );
    println!(
        "{}Print the user's username.  Same as without arguments.",
        "    username        ".bright_cyan(),
    );
    println!(
        "{}Print the host device's (pretty) name.",
        "    devicename      ".bright_cyan(),
    );
    println!(
        "{}Print the host device's hostname.",
        "    hostname        ".bright_cyan(),
    );
    println!(
        "{}Print the desktop environment.",
        "    desktop_env     ".bright_cyan(),
    );
    println!(
        "{}Print the operating system name and version.",
        "    distro          ".bright_cyan(),
    );
    println!(
        "{}Print the host platform.",
        "    platform        ".bright_cyan(),
    );
    println!(
        "{}Print the host architecture.",
        "    arch            ".bright_cyan(),
    );
    println!(
        "{}Print everything known by whome.\n",
        "    print           ".bright_cyan(),
    );
}

fn main() -> Result {
    let args = &mut ::std::env::args();

    if let Some(a) = args.nth(1) {
        if args.count() > 2 {
            println!("too many arguments, try `whome help`");
        } else {
            match a.as_str() {
                "help" | "--help" => help(),
                "version" | "--version" => version(),
                "realname" | "--realname" => println!("{}", whoami::realname()),
                "username" | "--username" => println!("{}", whoami::username()),
                // TODO: Set Hostname.
                "hostname" | "--hostname" => {
                    println!("{}", whoami::fallible::hostname()?)
                }
                "devicename" | "--devicename" => {
                    println!("{}", whoami::devicename())
                }
                "print" | "--print" => {
                    print!(
                        "realname:     {}\nusername:     {}\n\
                         devicename:   {}\nhostname:     {}\n\
                         distro:       {}\ndesktop_env:  {}\n\
                         platform:     {}\narch:         {}\n",
                        whoami::realname(),
                        whoami::username(),
                        whoami::devicename(),
                        whoami::fallible::hostname()?,
                        whoami::distro(),
                        whoami::desktop_env(),
                        whoami::platform(),
                        whoami::arch(),
                    );
                }
                "desktop_env" | "--desktop_env" => {
                    println!("{}", whoami::desktop_env())
                }
                "distro" | "--distro" => println!("{}", whoami::distro()),
                "platform" | "--platform" => println!("{}", whoami::platform()),
                "arch" | "--arch" => println!("{}", whoami::arch()),
                a => {
                    print!("Unknown Argument: {a}\n\n");
                    help();
                }
            }
        }
    } else {
        println!("{}", whoami::username()); // no args
    }

    Ok(())
}

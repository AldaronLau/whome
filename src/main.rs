// Who Me?
// Copyright © 2017-2020 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

extern crate term;
extern crate whoami;

fn version() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_RED).unwrap();
    write!(t, "aldaronlau.").unwrap();
    t.fg(term::color::BRIGHT_BLUE).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, env!("CARGO_PKG_NAME")).unwrap();
    t.reset().unwrap();
    write!(t, " ").unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    writeln!(t, env!("CARGO_PKG_VERSION")).unwrap();
    t.reset().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "Copyright ©").unwrap();
    t.reset().unwrap();
    write!(t, " ").unwrap();
    t.fg(term::color::MAGENTA).unwrap();
    writeln!(t, "Jeron Lau 2017 - 2020.").unwrap();
    t.reset().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "License ").unwrap();
    t.reset().unwrap();
    t.fg(term::color::MAGENTA).unwrap();
    writeln!(t, "MIT / BSL-1.0").unwrap();
    t.reset().unwrap();
}

fn help() {
    let mut t = term::stdout().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "Usage ").unwrap();
    t.reset().unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    write!(t, "whome ").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    writeln!(t, "[OPTION]").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the name of the user who is logged in.").unwrap();
    writeln!(t).unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    help            ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print this help and exit.").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    version         ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print version and exit").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    realname        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the user's full name.").unwrap();
    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    username        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the user's username.  Same as without arguments.")
        .unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    devicename      ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the host device's (pretty) name.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    hostname        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the host device's hostname.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    desktop_env     ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the desktop environment.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    distro          ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the operating system name and version.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    platform        ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print the host platform.").unwrap();

    t.fg(term::color::BRIGHT_CYAN).unwrap();
    write!(t, "    print           ").unwrap();
    t.reset().unwrap();
    writeln!(t, "Print everything known by whome.").unwrap();

    writeln!(t).unwrap();
}

fn main() {
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
                "hostname" | "--hostname" => println!("{}", whoami::hostname()),
                "devicename" | "--devicename" => println!("{}", whoami::devicename()),
                "print" | "--print" => {
                    print!(
                        "realname:     {}\nusername:     {}\n\
                         devicename:   {}\nhostname:     {}\n\
                         distro:       {}\ndesktop_env:  {}\n\
                         platform:     {}\n",
                        whoami::realname(),
                        whoami::username(),
                        whoami::devicename(),
                        whoami::hostname(),
                        whoami::distro(),
                        whoami::desktop_env(),
                        whoami::platform(),
                    );
                }
                "desktop_env" | "--desktop_env" => println!("{}", whoami::desktop_env()),
                "distro" | "--distro" => println!("{}", whoami::distro()),
                "platform" | "--platform" => println!("{}", whoami::platform()),
                a => {
                    print!("Unknown Argument: {}\n\n", a);
                    help();
                }
            }
        }
    } else {
        println!("{}", whoami::username()); // no args
    }
}

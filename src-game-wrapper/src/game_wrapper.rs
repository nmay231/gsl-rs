use clap::{arg, command, value_parser, Command as ClapCommand};
use nix::sys::signal;
use nix::unistd::Pid;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command as ProcCommand, Stdio};
use std::sync::{Arc, Mutex};

enum Message {
    TextLine(String),
    Kill,
}

pub fn run_game_wrapper() {
    let stdin = io::stdin();
    let stdin_queue = Arc::new(Mutex::new(VecDeque::new()));
    // let stdin_queue_thread = Arc::clone(&stdin_queue);
    // let thread_kill = OnceLock::new();
    // let stdin = Arc::new(Mutex::new(io::stdin()));
    // let stdin_thread = std::thread::spawn(move || loop {
    //     if thread_kill.get().is_none() {
    //         break;
    //     }
    //     let mut buf = String::new();
    //     stdin.read_line(&mut buf).expect("cannot read stdin");

    //     {
    //         let mut lock = stdin_queue.lock().expect("thread already held queue lock");
    //         (*lock).push_back(Message::TextLine(buf))
    //     }
    // });

    let matches = command!()
        .subcommand(
            ClapCommand::new("run")
                .about("Run a game server")
                .arg(arg!(--"game-server" <game_server>).value_parser(value_parser!(PathBuf))), // .arg(arg!(--game_stdin <game_stdin>).value_parser(value_parser!(PathBuf))),
        )
        .get_matches();

    let matches = matches
        .subcommand_matches("run")
        .expect("only the `run` subcommand is supported right now");

    println!("{:?}", (matches.ids().collect::<Vec<_>>()));

    let game_server = matches
        .get_one::<PathBuf>("game-server")
        .expect("`--game-server` is required");
    assert!(
        game_server.exists(),
        "game_server at {} does not exist",
        game_server.display()
    );

    // let game_stdin = matches
    //     .get_one::<PathBuf>("game-stdin")
    //     .expect("`--game-stdin` is required");
    // assert!(
    //     game_stdin.exists(),
    //     "game_stdin at {} does not exist",
    //     game_stdin.display()
    // );

    let mut subproc = ProcCommand::new("sh")
        .arg(game_server)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Could not spawn child process");
    println!("{}", 11);
    let stdin_queue2 = Arc::clone(&stdin_queue);
    let message_handler = std::thread::spawn(move || loop {
        {
            let queue = stdin_queue2.lock().unwrap();
            for message in queue.iter() {
                match message {
                    Message::TextLine(text) => {
                        subproc
                            .stdin
                            .as_mut()
                            .unwrap()
                            .write_all(text.as_bytes())
                            .unwrap();
                    }
                    Message::Kill => {
                        signal::kill(Pid::from_raw(subproc.id() as i32), signal::SIGINT).unwrap();
                        // subproc.kill()

                        // nix::sys::prctl::;
                    }
                }
            }
        }
        std::thread::park();
    });
    println!("{}", 22);
    // let message_handler = Arc::new(message_handler);
    // let message_handler2 = Arc::clone(&message_handler);
    // let stdin_queue2 = Arc::clone(&stdin_queue);
    // ctrlc::set_handler(move || {
    //     {
    //         (*stdin_queue2.lock().unwrap()).push_back(Message::Kill)
    //     }
    //     message_handler2.thread().unpark();
    // })
    // .unwrap();

    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("cannot read stdin");
        println!("{}", 33);

        {
            (*stdin_queue.lock().unwrap()).push_back(Message::TextLine(buf))
        }
        message_handler.thread().unpark();
    }
}

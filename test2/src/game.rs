use std::{thread::sleep, time::Duration, thread, io, io::Write, io::stdin, io::Error};
use std::sync::{Arc, Mutex, mpsc::channel, mpsc::Sender, mpsc::Receiver};

fn bad_line(line: String){
    println!("<{}> No es una comanda acceptada.", line);
    handle_cmd();
}


fn handle_cmd() -> Result<i32, Error>{
    let mut line = String::new();
    println!("lolxd?: ");
    stdin().read_line(&mut line)?;

    match line.trim(){
        "collect" => return Ok(1),
        _ => bad_line(line),
    };
    panic!("mal");//this panics ?¿
}

fn thread_handler(i: &mut i32, tx: Sender<i32>, rx: Receiver<i32>, mutex: &Arc<Mutex<i32>>){
    loop{
        *i += 1;
        sleep(Duration::from_secs(1));
    }
}

pub fn run_game(){
    let mut i = 0;
    let interval = Duration::from_secs(1);
    let main_mutex_pointer = Arc::new(Mutex::new(0));
    let (thread_Send, main_Recv) = channel();
    let (main_Send, thread_Recv) = channel();

    print!("Starting thread: \n");
    let thread_mutex_pointer = Arc::clone(&main_mutex_pointer);
    let _thread = thread::spawn(move || thread_handler(&mut i, thread_Send, thread_Recv, &thread_mutex_pointer));

    loop{
        println!("ADASD");
        io::stdout().flush().unwrap();
        //let received = main_Recv.recv().unwrap();
        let command = match handle_cmd(){
            Ok(command) => command,
            Err(e) => panic!("{}", e),
        };
        println!("command{}", command);

        //print!("\r{} ", received); //https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
        sleep(interval);
        break;
    }
    println!("DONE!")
}
use std::{thread::sleep, time::Duration, thread, io, io::Write, io::stdin, io::Error};
use std::sync::{Arc, Mutex, mpsc::channel, mpsc::Sender, mpsc::Receiver};

const COLLECT: &str= "collect";
const QUIT: &str = "quit";
 
#[derive(Debug)]
struct ThreadHandler{
    i: i128,
    tx: Sender<i128>,
    rx: Receiver<String>,
    mutex: Arc<Mutex<i128>>,
}
//self.tx.send(self.i).unwrap()
impl ThreadHandler{
    fn run(&mut self){
        loop{
            self.i += 1;
            match self.rx.try_recv(){
                Ok(command) => self.thread_commands_handler(command),
                _ => {} 
            };
            sleep(Duration::from_secs(1));
        }
    }

    fn thread_commands_handler(&mut self, command: String){
        match &*command{
            COLLECT => self.thread_handle_collect(),
            _ => panic!("thread_commands_handler panics"),
        }
    }

    fn thread_handle_collect(&mut self){
        self.tx.send(self.i).unwrap();
        self.i = 0;
    }
}

fn bad_line(line: String){
    println!("<{}> No es una comanda acceptada.", line);
    handle_cmd();
}

fn handle_cmd() -> Result<&'static str, Error>{
    let mut line = String::new();
    println!("Introduir comanda:");
    stdin().read_line(&mut line)?;

    match line.trim(){
        "collect" => return Ok(COLLECT),
        "quit" => return Ok(QUIT),
        _ => bad_line(line),
    };
    panic!("mal");
}

fn handle_collect(main_Recv: &Receiver<i128>, main_Send: &Sender<String>, points: &mut i128){
    main_Send.send(COLLECT.to_string()).unwrap();
    (*points) += main_Recv.recv().unwrap();
}

pub fn run_game(){
    let mut points = 0i128;
    let interval = Duration::from_secs(1);

    let main_mutex_pointer = Arc::new(Mutex::new(0)); //hauria de anar i aqui?¿
    let (thread_Send, main_Recv) = channel();
    let (main_Send, thread_Recv) = channel();
    
    let mut thread_handler = ThreadHandler{
        i: 0i128,
        tx: thread_Send,
        rx: thread_Recv,
        mutex: Arc::clone(&main_mutex_pointer),
    };
    print!("Starting thread...\n");
    
    let _thread = thread::spawn(move || thread_handler.run());

    loop{
        io::stdout().flush().unwrap();

        let command = match handle_cmd(){
            Ok(command) => match command{
                COLLECT => handle_collect(&main_Recv, &main_Send, &mut points),
                QUIT => {
                    println!("Quitting...");
                    break;
                },
                _ => panic!(),
            },
            Err(e) => panic!("{}", e),
        };
        println!("You have {} points.", points);
        //https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
        sleep(interval);
    }
    println!("DONE!")
}
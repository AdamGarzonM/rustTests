//prob no hauria de retornar String
fn parse_arguments() -> String{
    let args: Vec<String> = env::args().collect();

    path = &args[1];
    //maybe aquest read no es lo millor? (maybe un linea x linea)
    let contents = fs::read_to_string(path)?;
    return contents;
}

fn main(){
    let input = match parse_arguments(){
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
}
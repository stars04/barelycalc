/*
 * Idea is to store numbers in one vec and operators in another vec and use them
 * to build out the order of operations 
 */
pub fn exporder(iexpr: String, eexpr: String) -> String {
    let init: String = iexpr;
    let operators: [char; 4] = ['*', '/', '+', '-'];
    let mut hput: String = eexpr; 
    let mut oploc: usize = 0;
    let mut op: String = String::new();
    let mut containerl: String = String::new();
    let mut containerr: String = String::new();
    if init.contains(operators[0]) {
        oploc = init.find('*').unwrap();
        op = operators[0].to_string();
    }
    else if init.contains(operators[1]) {
        oploc = init.find('/').unwrap();
        op = operators[1].to_string();
    }
    else if init.contains(operators[2]) {
        oploc = init.find('+').unwrap();
        op = operators[2].to_string();
    }
    else if init.contains(operators[3]) {
        oploc = init.find('-').unwrap();
        op = operators[3].to_string();
    }
    containerl = init.get(..oploc).unwrap().to_string();
    println!("Containerl => {:?}", &containerl);
    if init.len() > oploc+1 {
        containerr = init.get(oploc+1..).unwrap().to_string();
        println!("Containrr => {:?}", &containerr);
    } else {
        containerr = String::new();
    }
    //let mut containerr = init.get(oploc+1..).unwrap().to_string();
    
    for char in containerl.chars().rev() {
        if operators.contains(&char) {
            oploc = containerl.rfind(char).unwrap()+1;
            println!("THE VAL OF OPLOC IN CONTL => {:?}", &oploc);
            containerl = containerl.get(oploc-1..).unwrap().to_string();
            break;
        } else {
            continue;
        }
    }
    for char in containerr.chars() {
        if operators.contains(&char) {
            oploc = containerr.find(char).unwrap();
            containerr = containerr.get(..oploc).unwrap().to_string();
            break;
        }
    }
    /* Currently Have a _MOSTLY_ working expression parser
     * Challenge is currently that in the case of rearranging symbols, two numbers are joined
     * when they should not 
     * Currenlty 5+1+2* => *25+1+ | Need some way to consistantly output correct expression order
    */
    println!("Containerl => {:?}", &containerl);
    println!("Containrr => {:?}", &containerr);
    let mut temp = String::new();
    let temp1 = containerl.clone() + &op + &containerr;
    if !containerr.is_empty() && !containerl.is_empty(){
        temp = containerl + &op + &containerr;
    }else if containerl.is_empty() {
        temp = op + &containerr;
    }else if containerr.is_empty() {
        temp = op + &containerl;
    }
    let rem = init.replace(&temp1, "");
    let mut svec: Vec<char> = Vec::new();
    let mut hvec: Vec<char> = Vec::new();
    println!("VALUE OF TEMP BEFORE IF STATEMENT {:?}\n VALUE OF TEMP1 {:?}", temp, temp1);
    if operators.contains(&temp1.clone().chars().next().unwrap()) && operators.contains(&temp1.chars().nth(temp1.len()-1).unwrap()) {
        println!("========================================================================================\nMET OUR CONDITION\n=====================================================================");
        let posopl = operators.iter().position(|&r| r == temp1.chars().next().unwrap()).unwrap();
        let posopr = operators.iter().position(|&r| r == temp1.chars().nth(temp1.len()-1).unwrap()).unwrap();
        if posopr < posopl {
            let mut svec: Vec<char> = Vec::new();
            let mut text: String = String::new();
            for chars in temp1.chars() {
                svec.push(chars);
            }
            let first: char = svec.pop().unwrap();
            println!("first {:?}", first);
            let last: char = svec[0];
            println!("last {:?}", last);
            text.push(first);
            svec.remove(0);
            for char in svec {
                text.push(char);
            }
            text.push(last);
            println!(" THE FINAL VALUE OF TEXT ===================================++>{:?}", text);
            hput.push_str(&text); 
        }
        else {
            hput.push_str(&temp1);
        }
    }
        else {
        println!("===____ EXTERIOR ELSE BRANCH INVOKED _____===");
        hput.push_str(&temp);
    }
    println!("===> PLEASE SEE MEE <===");
    println!("what remains => {:?}\nwhat is output => {:?}", rem, &hput);
    if !rem.is_empty() {
        exporder(rem, hput)
    }else {
        hput
    }
}
pub fn ordparser(input: String, order: Vec<String>) -> Vec<String> {
    let mut ops: Vec<String> = order;
    let mut inp0: String = input;
    let mut inp1: String = inp0.clone();
    let mut intr: String = String::new();

    if inp0.contains("(") {
        let operators: [char; 4] = ['*', '/', '+', '-'];
        let mut container: String = String::new();
        let mut container1: String = String::new();
        let mut containvec: Vec<String> = Vec::new();
        let mut offset = inp0.rfind('(').unwrap();
        let mut rffset = inp0.find(')').unwrap() + 1;
        inp0.replace_range(offset..rffset, "");
        inp1 = inp1.get(offset..rffset).unwrap().to_string();
        offset = inp1.find('(').unwrap() + 1;
        rffset = inp1.find(')').unwrap();
        inp1 = inp1.get(offset..rffset).unwrap().to_string();
        println!("**********AT START OF EXPORDER => {:?}************", &inp1);
        inp1 = exporder(inp1, container1);
        println!("After exporder => {:?}", &inp1);

        for char in inp1.clone().chars() {
            inp1.remove(0);
            if char != '+' && char != '-' && char != '*' && char != '/' {
                container.push(char);
                if inp1.is_empty() {
                    ops.push(container.clone());
                }
            }else {
                if !container.is_empty() {
                    ops.push(container.clone())
                }
                ops.push(char.to_string());
                container = String::new();
            }
        }
    }
    if !inp0.is_empty() {
        ordparser(inp0, ops)
    } else {
        ops
    }
}

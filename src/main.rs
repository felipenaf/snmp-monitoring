use std::thread;
mod command;

fn main() {
    // gambiarra pra não dar warning
    if false {command::ls();} 
    if false {command::snmpget(0);} 

    let numbers = vec![0;20];

    if true {
        mythread(numbers);
    } else {
        myqueue(numbers);
    }
}

fn myqueue (numbers: Vec<i32>) {
    for (i, number) in numbers.iter().enumerate() {
        if false {println!("{}", number);} 

        command::snmpget(i + 1);
    }
}

fn mythread (numbers: Vec<i32>) {
    let mut handles = vec![];

    for (i, number) in numbers.iter().enumerate() {
        if false {println!("{}", number);} 
        
        let handle = thread::spawn(move || {
            command::snmpget(i + 1);
        });

        handles.push(handle);
    }
    
    for handle in handles {
        // each handle go to some a difirent thread
        handle.join().unwrap();
    }

    println!("All threads finished");
}
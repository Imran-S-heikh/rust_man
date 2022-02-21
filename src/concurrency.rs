use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}};

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("HI number {i} spawn thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi, number {i} from main thread!");
        thread::sleep(Duration::from_millis(1))
    }

    {
        let name = String::from("Imran Shaikh");

        let handle = thread::spawn( move ||{
            println!("{name}");
        });

        // println!("{name}");

        handle.join().unwrap();
    }

    {
        let (tx,rx) = mpsc::channel();

        thread::spawn(move||{
            let val = String::from("Hello");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    {
        let (tx,rx) = mpsc::channel();

        thread::spawn(move||{
            let vals = vec![
                String::from("Hello"),
                String::from("World"),
                String::from("I am, "),
                String::from("Imran From Bangladesh")
            ];

            for val in vals {
                tx.send(val).unwrap();
                // thread::sleep(Duration::from_millis(1));
            }
        });

        for received in rx {
            println!("Got From Multiple: {received}"); 
        }
        
    }

    {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}",m);
    }

    {
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move||{
                let mut num = counter.lock().unwrap();
                
                *num += 1;
            });
            
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }


        println!("Result: {}", *counter.lock().unwrap());
    }

}

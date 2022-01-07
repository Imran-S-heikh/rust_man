pub fn start() {
    let months = [
        "Janunary",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let birthday_month = months[11];
    let birthday_date:u8 = 27;
    let birthday = format!("{} {}", birthday_month, birthday_date);

    'main_loop: for month in months {
        let mut current_day: u8 = 1;

        println!("Current Month is {}",month);

        loop {
            if current_day > 30 {
                break;
            }

            println!("Current Date is {}",current_day);

            let current_date = format!("{} {}", month, current_day);

            if current_date == birthday {
                println!("It's {}",current_date);
                println!("Happy Birthday!!");
                break 'main_loop;
            }
            current_day += 1;
        }
    }
}

pub fn loop_returns_value() {
    let mut number = 1;
    let result: u8 = loop {
        if number == 5 {
           break 5* 10;
        }
        number+=1;
    };

    println!("Result Should be equel to 50, Result: {}",result);
}

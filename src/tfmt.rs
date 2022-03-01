use std::fmt::format;

struct Unit {
    second: usize,
    minute: usize,
    hour: usize,
    day: usize,
    year: usize,
}

struct HandleTimeProps {
    unit: usize,
    label: String
}

pub struct TimeFormat {
    time: usize,
    result: Vec<String>,
    unit: Unit,
}

impl TimeFormat {
    pub fn new(time: usize) -> TimeFormat {
        let second = 1000;
        let minute = second * 60;
        let hour = minute * 60;
        let day = hour * 24;
        let year = day * 365;

        TimeFormat {
            time,
            result: vec![],
            unit: Unit {
                second,
                minute,
                hour,
                day,
                year,
            },
        }
    }

    pub fn get_years(mut self,label: &str) -> TimeFormat {
        let params = HandleTimeProps {
            unit: self.unit.year,
            label: String::from(label)
        };

        self.handle_time(params);

        self
    }

    pub fn get_days(mut self,label: &str) -> TimeFormat {
        let params = HandleTimeProps {
            unit: self.unit.day,
            label: String::from(label)
        };

        self.handle_time(params);

        self
    }

    fn handle_time(&mut self, params: HandleTimeProps) {
        let HandleTimeProps {unit,label} = params;
        let time = self.time / unit;

        if time > 0 {
            let time = format!("{time}{label}");
            self.result.push(time);
            self.time = self.time % unit;
        }
    }

    pub fn format(self) -> String {
        self.result.join("")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn initialize() {
        let fmt = TimeFormat::new(32424);
    }

    #[test]
    fn test_year_and_day() {
        // 31968000000 = 1 Year 5 Days => 15
        let time = TimeFormat::new(31968000000).get_years("Year").get_days("Day").format();
        assert_eq!(time, "1Year5Day");
    }
}

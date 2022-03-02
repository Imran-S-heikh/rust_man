use std::fmt::format;

struct Units {
    second: usize,
    minute: usize,
    hour: usize,
    day: usize,
    year: usize,
}

struct HandleTimeProps {
    unit: usize,
    label: String,
}

pub struct TimeFormat {
    time: usize,
    result: Vec<String>,
    label: Labels,
    unit: Units,
    space: bool,
    plural: bool
}

struct Format {
    unit: usize,
    label: String,
}

struct Labels {
    second: String,
    minute: String,
    hour: String,
    day: String,
    month: String,
    year: String,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            second: String::from("Second"),
            minute: String::from("Minute"),
            hour: String::from("Hour"),
            day: String::from("Day"),
            month: String::from("Month"),
            year: String::from("Year"),
        }
    }
}

pub struct TimeFormatOptions {
    space: bool,
    plural: bool,
    labels: Labels,
}

impl Default for TimeFormatOptions {

    fn default() -> Self {
        Self {
            space: false,
            plural: true,
            labels: Labels::default(),
        }
    }
}


impl TimeFormat {
    pub fn new(time: usize,options: TimeFormatOptions) -> TimeFormat {
        let TimeFormatOptions { space,labels,plural } = options;
        let second = 1000;
        let minute = second * 60;
        let hour = minute * 60;
        let day = hour * 24;
        let year = day * 365;

        TimeFormat {
            time,
            space,
            plural,
            result: vec![],
            label: labels,
            unit: Units {
                second,
                minute,
                hour,
                day,
                year,
            },
        }
    }

    pub fn get_years(mut self) -> TimeFormat {
        let params = HandleTimeProps {
            unit: self.unit.year,
            label: self.label.year.clone(),
        };

        self.handle_time(params);

        self
    }

    pub fn get_days(mut self) -> TimeFormat {
        let params = HandleTimeProps {
            unit: self.unit.day,
            label: self.label.day.clone(),
        };

        self.handle_time(params);

        self
    }

    fn handle_time(&mut self, params: HandleTimeProps) {
        let HandleTimeProps { unit, label } = params;
        let time = self.time / unit;

        if time > 0 {
            let space = match self.space {
                true => " ",
                false => ""
            };
            let time = if time == 1 {
                format!("{time}{space}{label}")
            }else {
                format!("{time}{space}{label}s")
            };
            self.result.push(time);
            self.time = self.time % unit;
        }
    }

    pub fn format(self, sep: &str) -> String {
        self.result.join(sep)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn initialize() {
        let options = TimeFormatOptions::default();
        let fmt = TimeFormat::new(32443,options);
    }

    #[test]
    fn test_year_and_day() {
        // 31968000000 = 1 Year 5 Days => 15
        let options = TimeFormatOptions::default();
        let time = TimeFormat::new(31968000000,options)
            .get_years()
            .get_days()
            .format(" ");
        assert_eq!(time, "1Year 5Days");
    }
}

use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

fn main(){
    let input = include_str!("test_input.txt");
    let mut records = parse_input(input);
    records.sort();

    println!("{:?}", sleepiest_guard(records));
}

fn parse_input(input: &str) -> Vec<Record>{
    input.lines().map(|l| l.parse::<Record>().ok().unwrap() ).collect()
}

fn sleepiest_guard(records: Vec<Record>) -> GuardID {
    let mut sleep_counter = HashMap::new();
    
    let mut current_guard = Guard::Awake { id: 0 };

    let mut recs = records.iter();
    while let Some(r) = recs.next() {
        match &r.action {
            Action::BeginsShift(g) => current_guard = g.clone(),
            Action::FallsAsleep => {
                if let Guard::Awake{ id } = current_guard {
                    let bedtime = get_bedtime(r.time.clone());
                    current_guard = Guard::Sleeping { 
                                        id: id,
                                        since: bedtime.minute }
                }
            },
            Action::WakesUp => {
                if let Guard::Sleeping { id, since } = current_guard {
                    let duration = (since..r.time.minute).len();
                    {
                        let mut counter = sleep_counter.entry(id).or_insert(0);
                        *counter += duration;
                    }
                    current_guard = Guard::Awake{ id }
                }
            }
        }
    }

    let max_sleeper = sleep_counter.values().max().unwrap();
    let (grd, tm) = sleep_counter.iter().find(|(k,v)| *v == max_sleeper ).unwrap();

    *grd
}

fn get_bedtime(time: TimeStamp) -> TimeStamp {
    if time.hour == 0 {
        time
    } else {
        TimeStamp { 
            year: time.year,
            month: time.month,
            day: time.day,
            hour: 0,
            minute: 0
        }
    }
}

type Year = usize;
type Month = usize;
type Day = usize;
type Hour = usize;
type Minute = usize;

type GuardID = usize;

#[derive(Debug,Eq,Ord,PartialOrd,PartialEq,Clone)]
enum Guard {
    Sleeping{ id: GuardID, since: Minute },
    Awake{ id: GuardID }
}

#[derive(Debug,Eq,Ord,PartialOrd,PartialEq)]
struct Record {
    time: TimeStamp,
    action: Action
}

impl FromStr for Record {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let record = Record {
            time: s[..18].parse()?,
            action: s[19..].parse()?
        };
        Ok(record)
    }
}

#[derive(Debug,Eq,Ord,PartialOrd,PartialEq)]
enum Action {
    BeginsShift( Guard ),
    FallsAsleep,
    WakesUp
}

impl FromStr for Action {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        return match s {
            "wakes up" => Ok(Action::WakesUp),
            "falls asleep" => Ok(Action::FallsAsleep),
            _ => {
                let g = parts.nth(1).expect("Can't parse action");
                let guard = Guard::Awake{id: g[1..].parse::<GuardID>()? };
                Ok(Action::BeginsShift( guard ))
            }
        }
    }
}

#[derive(Debug,Eq,Ord,PartialOrd,PartialEq,Clone)]
struct TimeStamp {
    year: Year,
    month: Month,
    day: Day,
    hour: Hour,
    minute: Minute
}

impl FromStr for TimeStamp {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = TimeStamp {
            year:   s[1..5].parse::<usize>()?,
            month:  s[6..8].parse::<usize>()?,
            day:    s[9..11].parse::<usize>()?,
            hour:   s[12..14].parse::<usize>()?,
            minute: s[15..17].parse::<usize>()?,
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sleepiest_guard() {
        let input = include_str!("test_input.txt");
        let mut records = parse_input(input);
        records.sort();

        assert_eq!(sleepiest_guard(records), 10);
        // assert_eq!(sleepiest_minute(records), 24);
    }
}
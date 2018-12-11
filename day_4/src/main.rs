use std::io::BufReader;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fs::File;
extern crate chrono;
use chrono::prelude::*;

extern crate regex;
use regex::Regex;
// Helpers
fn open_input() -> std::io::BufReader<std::fs::File> {
    let file = match File::open("./assets/input") {
            Err(why) => panic!("couldn't open, {}", why),
            Ok(file) => file,
        };
    BufReader::new(file)
}

pub trait SantaEpoch {
    fn santa_epoch (self) -> i64;
}

impl SantaEpoch for DateTime<Utc> {
    fn santa_epoch(self) -> i64 {
        // Returns number of seconds since Santa's Birth
        let santa_birth = Utc.ymd(270, 3, 15).and_hms(0, 0, 0);
        self.signed_duration_since(santa_birth).num_seconds()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Task {
    BeginShift,
    WakeUp,
    FallSleep,
    Undefined,
}

#[derive(Debug, Eq)]
struct Event {
    event_type: Task,
    guard_id: u32,
    date: DateTime<Utc>,
    epoch: i64,
}

impl Ord for Event {
     fn cmp(&self, other: &Event) -> Ordering {
         self.epoch.cmp(&other.epoch)
     }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.epoch == other.epoch
    }
}


#[derive(Debug, Default)]
struct Guard {
     id: u32,
     sleep: i64,
     sleep_freq: HashMap<u32, u32>,
     log: Vec<Event>,
 }


impl Guard {
    fn minute_possible_sleep (&self) -> &u32 {
        //Return the most frequent minute guard is sleeping
        match self.sleep_freq.iter().max_by_key(|&(_k, v)| v) {
        Some(result) => {
            result.0
        },
        None => &0u32,
        }
    }
}

impl Ord for Guard {
    fn cmp(&self, other: &Guard) -> Ordering {
        self.sleep.cmp(&other.sleep)
    }
    fn max(self, other: Self) -> Self
        where Self: Sized {
        if other.sleep >= self.sleep { other } else { self }
    }
}

impl PartialOrd for Guard {
    fn partial_cmp(&self, other: &Guard) -> Option<Ordering> {
        Some(self.sleep.cmp(&other.sleep))
    }
}


impl PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
        self.sleep == other.sleep
    }
}

impl Eq for Guard {}

impl Hash for Guard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn get_guard_data() -> HashMap<u32, Guard> {
    let mut event_log : BinaryHeap<Event> = BinaryHeap::new();
    let mut guards_sum: HashMap<u32, Guard> = HashMap::new();

    let re = Regex::new(
        r"(?si)\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] (?P<event>(wakes|falls|Guard)) \s*(\#(?P<id>\b[0-9]+))?").unwrap();
    for line in open_input().lines() {
        let str_line = line.unwrap().clone();
        let guards_string = re.captures(&str_line).unwrap();
        
        let year = guards_string["year"].parse::<i32>().unwrap();
        let month = guards_string["month"].parse::<u32>().unwrap();
        let day = guards_string["day"].parse::<u32>().unwrap();
        let hour = guards_string["hour"].parse::<u32>().unwrap();
        let minute = guards_string["minute"].parse::<u32>().unwrap();
        let event = guards_string["event"].parse::<String>().unwrap();
        
        let date = Utc.ymd(year, month, day).and_hms(hour, minute, 0);
        let mut guard_id = 0;

        let mut event_type = Task::Undefined;
        if event == "falls" {
            event_type = Task::FallSleep;
            guard_id = 0; // Undefined, list is not sorted yet
        }else if event == "wakes" {
            event_type = Task::WakeUp; 
            guard_id = 0; // Undefined, list is not sorted yet
        } else if event == "Guard" {
            guard_id = guards_string["id"].parse::<u32>().unwrap();
            event_type = Task::BeginShift;
        }

        let log =  Event{ event_type, guard_id, date, epoch: date.santa_epoch() };
        event_log.push(log);
    }

    let mut id = 0;
    let mut temp_date_time = Utc.ymd(1518, 3, 4).and_hms(0, 0, 0); 
    for x in event_log.into_sorted_vec().iter() {
        if x.guard_id > 0 {
            id = x.guard_id;
            let guard_st: Guard = Guard { id, log: vec![], sleep_freq: HashMap::new(), sleep:0 };
            guards_sum.entry(id).or_insert(guard_st); 
        } else {
            id = id;
        }
        if x.event_type == Task::FallSleep {
             temp_date_time = x.date;
        } else if x.event_type == Task::WakeUp {
           let minutes = x.date.signed_duration_since(temp_date_time).num_minutes();

            match guards_sum.get_mut(&id) {
                Some(guard) => {
                        guard.sleep +=minutes;
                        for minute_tic in 0..minutes {
                            let tics = (temp_date_time + chrono::Duration::minutes(minute_tic)).minute();
                            *guard.sleep_freq.entry(tics).or_insert(0) += 1;
                        }
                    },
                 None => println!("Error Guard not Found"),
            }
        }
        match guards_sum.get_mut(&id) {
                Some(guard) => guard.log.push(Event{date: x.date, guard_id: x.guard_id, event_type: x.event_type.clone(), epoch: x.epoch}),
                 None => println!("Error Guard not Found")
            }
    }

    guards_sum
}

fn part1(guard_data: &HashMap<u32, Guard>) -> (String ,u32) {
    let sleepy_guard = guard_data.iter().max_by_key(|&(_k, v)| v).unwrap().1;
    let guard_id = sleepy_guard.id;
    let minute_sleep = sleepy_guard.minute_possible_sleep();
    let total_sleep = sleepy_guard.sleep;
    let description = format!("Guard id {}, most likely sleeping at 00:{}, was sleeping {} minutes totaly", guard_id, minute_sleep, total_sleep);
    
    (description, sleepy_guard.id*minute_sleep)
}

fn part2(guard_data: &HashMap<u32, Guard>) -> (String, u32) {
    let mut most_minute_guard: [&u32; 3] = [&0u32,&0u32,&0u32];
    for (guard_id,guard) in guard_data.iter() {

        if let Some(result) = guard.sleep_freq.iter().max_by_key(|&(_k, v)| v) {
            if result.1 > most_minute_guard[2] {
                most_minute_guard = [guard_id, result.0 ,result.1];
            }
        }
    }
    let description = format!("Guard id {}, was sleeping at 00:{}, {} times", most_minute_guard[0], most_minute_guard[1], most_minute_guard[2]);
    
    (description, most_minute_guard[0]*most_minute_guard[1])
}

fn main() {
    let guard_data = get_guard_data();
    println!("Results for Day 4");
    println!("-------------------");
    let (desc_text_1, answer_1) = part1(&guard_data);
    let (desc_text_2, answer_2) = part2(&guard_data);
    println!("Part1: {}",desc_text_1);
    println!("Part2: {}",desc_text_2);
    println!("Part 1: Answer: {}", answer_1);
    println!("Part 2: Answer: {}", answer_2);
}

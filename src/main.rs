use plotters::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

mod data;
use data::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Campus {
    North,
    Centinnial,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
struct Time {
    hour: u32,
    minute: u32,
}

impl Time {
    fn into_min(&self) -> u32 {
        self.hour * 60 + self.minute
    }

    fn from_min(min: u32) -> Self {
        Self {
            hour: min / 60,
            minute: min % 60,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.hour, self.minute))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hour > other.hour {
            std::cmp::Ordering::Greater
        } else if self.hour < other.hour {
            std::cmp::Ordering::Less
        } else {
            if self.minute > other.minute {
                std::cmp::Ordering::Greater
            } else if self.minute < other.minute {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[s.len() - 5..];

        assert!(s.len() == 5);

        let hour = s[..2].parse().unwrap();
        let minute = s[3..].parse().unwrap();

        Ok(Self { hour, minute })
    }
}

trait Range {
    fn overlap(&self, other: &Self) -> bool;
}

impl Range for TimeRange {
    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl Range for Day {
    fn overlap(&self, other: &Self) -> bool {
        self == other
    }
}

impl Range for DayTime {
    fn overlap(&self, other: &Self) -> bool {
        self.meet_day.overlap(&other.meet_day)
            && self.time.overlap(&other.time)
            && self.campus == other.campus
    }
}

impl Range for Vec<DayTime> {
    fn overlap(&self, other: &Self) -> bool {
        self.iter().any(|x| other.iter().any(|y| x.overlap(y)))
    }
}

impl Range for Section {
    fn overlap(&self, other: &Self) -> bool {
        self.meetings.overlap(&other.meetings)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct TimeRange {
    start: Time,
    end: Time,
}

impl TimeRange {
    fn len(&self) -> u32 {
        self.end.into_min() - self.start.into_min()
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

impl Add<i32> for Time {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Time::from_min((self.into_min() as i32 + rhs).max(0) as u32)
    }
}

impl Add<u32> for Time {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Time::from_min(self.into_min() + rhs)
    }
}

impl Sub<i32> for Time {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Time::from_min((self.into_min() as i32 - rhs).max(0) as u32)
    }
}

impl Sub<u32> for Time {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Time::from_min(self.into_min() - rhs)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rating {
    percent_a: f64,
    count_a: u64,
    count: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Section {
    class: String,
    section: String,

    facility: String,

    meetings: Vec<DayTime>,

    professor: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct DayTime {
    meet_day: Day,
    time: TimeRange,
    campus: Campus,
}

// const FILES_NAMES: &[&str; 5] = &[
//     /*"e101",*/ "csc216", "csc217", "csc226", "ma305", "py208", /*"py209",*/
// ];
// const FILES_NAMES: &[&str; 6] = &[
//     /*"e101",*/ "csc216", "csc217", "csc226", "ma305", "py208", "py209",
// ];
// const FILES_NAMES: &[&str; 7] = &[
//     "e101", "csc216", "csc217", "csc226", "ma305", "py208", "py209",
// ];
// const FILES_NAMES: &[&str; 6] = &["bme201", "bme209", "bme298", "ch223", "ch224", "ma341"];
//const FILES_NAMES: &[&str; 5] = &["csc230", "csc316", "csc333", "heso258", "ma341"];
// const FILES_NAMES: &[&str; 7] = &[
//     "mae206", "py208", "ma242", "mae200", "st370", "py209", "csc113",
// ];

const FILES_NAMES: &[&str; 3] = &["csc246", "csc326", "csc379"];

fn main() {
    let mut classes: Vec<Vec<Section>> = FILES_NAMES.iter().map(|&file| class_data(file)).collect();

    let mut acc = 1;

    for v in classes.iter() {
        acc *= v.len();
        println!("{:?}: {:?}", v[0].class, v.len());
    }

    // NOTE: Filter
    for v in classes.iter_mut() {
        v.retain(|x| {
            // No online classes
            if x.meetings.len() == 0 {
                return false;
            }

            // Payload Meeting
            // This is too unreliable
            // let payload = x.meet_days.contains(&Day::Wed)
            //     && x.time.overlap(&TimeRange {
            //         start: Time {
            //             hour: 12 + 3,
            //             minute: 0,
            //         },
            //         end: Time {
            //             hour: 12 + 4,
            //             minute: 0,
            //         },
            //     });

            // Late classes
            // let late = x
            //     .meetings
            //     .iter()
            //     .any(|m| m.time.end.hour >= 12 + 6 && x.class != "PY 208");
            let late = false;

            // Early classes
            //let early = x.time.start.hour <= 8;
            // let early = x.time.start.hour <= 8 && x.class != "PY 209" /*&& x.class != "CSC 216"*/;
            //let early = false;
            //

            // let mut early = x.time.start.hour <= 8 && x.campus == Campus::Centinnial;
            //let early = x.meetings.iter().any(|m| m.time.start.hour <= 8);
            let early = false;

            // :(
            // if x.class == "CSC 216" || x.class == "PY 209" {
            // if x.class == "CSC 216" || x.class == "PY 209" {
            //     early = false
            // };

            // No 8:30's on mondays
            // if !x.meet_days.contains(&Day::Mon) {
            //     early = false;
            // }
            // if x.class == "CSC 216" {
            //     early = false;
            // }

            // No friday classes
            let fri = x.meetings.iter().any(|m| m.meet_day == Day::Fri);
            // let fri = false;

            !(late || early || fri)
        })
    }

    println!("After filter");
    for v in classes.iter() {
        acc *= v.len();
        println!("{:?}: {:?}", v[0].class, v.len());
    }

    println!("Total possible {acc}");

    let solutions = possible_schedules(&classes);

    if solutions.len() == 0 {
        eprintln!("No solutions found!");
    } else {
        println!(
            "{} Solutions found",
            solutions.iter().flat_map(Tree::all_paths).count()
        );
        draw_classes(&solutions).unwrap()
    }
}

fn draw_classes(solutions: &Vec<Tree>) -> Result<(), Box<dyn std::error::Error>> {
    _ = std::fs::create_dir("output_images");

    for (i, data) in solutions.iter().flat_map(Tree::all_paths).enumerate() {
        let name = format!("output_images/{i}.png");
        let root = BitMapBackend::new(&name, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let w_count = 5;
        let h_count = 11;
        let h_start = 8;

        // The following code will create a chart context
        let mut chart = ChartBuilder::on(&root)
            .caption("Schedule", ("Arial", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                0.5f32..(w_count as f32 + 0.5) as f32,
                ((h_start + h_count) as f32)..h_start as f32,
            )?;

        chart
            .configure_mesh()
            .y_labels(h_count + 1)
            .y_label_formatter(&|&x| {
                let am = x <= 12.0;
                format!(
                    "{:0.} {}",
                    if am { x } else { x - 12.0 },
                    if am { "AM" } else { "PM" }
                )
            })
            .x_label_formatter(&|&x| {
                if x.fract() != 0.0 {
                    return "".to_string();
                }

                match x as i32 {
                    1 => "Mon",
                    2 => "Tue",
                    3 => "Wed",
                    4 => "Thu",
                    5 => "Fri",
                    _ => "",
                }
                .to_string()
            })
            .light_line_style(&TRANSPARENT)
            .disable_x_mesh()
            .draw()?;

        let (w, h) = chart.plotting_area().dim_in_pixel();
        let size = |len| {
            let width = w as i32 / (w_count);
            let height = h as f32 / (h_count as f32) * len;
            [(-width / 2, 0), (width / 2, height as i32)]
        };

        chart.draw_series(data.iter().flat_map(|x| {
            x.meetings.iter().map(|meeting| {
                let size = size(meeting.time.len() as f32 / 60.);

                let pos_x = match meeting.meet_day {
                    Day::Mon => 1.0,
                    Day::Tue => 2.0,
                    Day::Wed => 3.0,
                    Day::Thu => 4.0,
                    Day::Fri => 5.0,
                };

                let pos_y = meeting.time.start.hour as f32 + meeting.time.start.minute as f32 / 60.;

                let color = match meeting.campus {
                    Campus::North => RGBColor(200, 150, 150),
                    Campus::Centinnial => RGBColor(150, 150, 200),
                };

                EmptyElement::at((pos_x, pos_y))
                    + Rectangle::new(size, color.filled())
                    + Rectangle::new(size, RGBColor(255, 255, 255).stroke_width(2))
                    + Text::new(
                        format!("{} {}", x.class, x.section),
                        (size[0].0 + 2, size[0].1 + 2),
                        ("sans-serif", 15).into_font(),
                    )
                    + Text::new(
                        x.professor.split(' ').next().unwrap().to_owned(),
                        (size[0].0 + 2, size[0].1 + 2 + 15),
                        ("sans-serif", 15).into_font(),
                    )
            })
        }))?;
    }
    Ok(())
}

#[derive(Debug)]
struct Tree<'a> {
    value: &'a Section,
    children: Vec<Tree<'a>>,
}

impl Tree<'_> {
    fn depth(&self) -> usize {
        let mut count = 0;
        let mut curr = self;

        while curr.children.len() > 0 {
            count += 1;
            curr = &curr.children[0];
        }

        count
    }

    fn all_paths(&self) -> AllPaths {
        AllPaths {
            tree: self,
            current: vec![0; self.depth()],
        }
    }
}

#[derive(Debug)]
struct AllPaths<'a> {
    tree: &'a Tree<'a>,
    current: Vec<usize>,
}

impl<'a> Iterator for AllPaths<'a> {
    type Item = Vec<&'a Section>;

    fn next(&mut self) -> Option<Self::Item> {
        // Are we finished?
        if self.current[0] == self.tree.children.len() {
            return None;
        }

        // Get next vec
        let mut res = Vec::new();
        let mut lens = Vec::new();

        let mut curr = self.tree;
        for &x in &self.current {
            res.push(curr.value);
            lens.push(curr.children.len());

            curr = &curr.children[x];
        }
        res.push(curr.value);
        lens.push(curr.children.len());

        // Add one with overflow
        let len = self.current.len();
        self.current[len - 1] += 1;

        for i in (1..len).rev() {
            if self.current[i] >= lens[i] {
                self.current[i] = 0;
                self.current[i - 1] += 1;
            }
        }

        Some(res)
    }
}

fn possible_schedules_recursive<'a>(
    courses: &'a Vec<Vec<Section>>,
    history: &mut Vec<usize>,
) -> Option<Vec<Tree<'a>>> {
    let index = history.len();
    if index == courses.len() {
        // base case
        return Some(Vec::new());
    }

    let res: Vec<_> = courses[index]
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            history
                .iter()
                .enumerate()
                .all(|(i, &j)| practical(&v, &courses[i][j]))
        })
        .collect();

    // TODO: Ideally this wouldn't be required

    let res: Vec<Tree> = res
        .into_iter()
        .filter_map(|(i, v)| {
            history.push(i);
            let res = possible_schedules_recursive(courses, history);
            assert_eq!(Some(i), history.pop());

            res.map(|res| Tree {
                value: v,
                children: res,
            })
        })
        .collect();

    if res.len() == 0 {
        None
    } else {
        Some(res)
    }
}

fn possible_schedules<'a>(courses: &'a Vec<Vec<Section>>) -> Vec<Tree<'a>> {
    possible_schedules_recursive(courses, &mut Vec::new()).unwrap()
}

fn practical(this: &Section, other: &Section) -> bool {
    let overlap = this.overlap(other);

    let campus_travel = {
        this.meetings.iter().any(|x| {
            let expanded_range = TimeRange {
                start: x.time.start - 30,
                end: x.time.end + 30,
            };

            other
                .meetings
                .iter()
                .filter(|y| y.meet_day == x.meet_day)
                .filter(|y| y.time.overlap(&expanded_range))
                .any(|y| x.campus != y.campus)
        })
    };

    !(overlap || campus_travel)
}

#[allow(unused)]
fn load_classes() -> Vec<Vec<Section>> {
    FILES_NAMES
        .iter()
        .map(|file_name| {
            let reader = File::open(format! {"output/{file_name}.json"}).expect("File must exist");
            serde_json::from_reader(reader).unwrap()
        })
        .collect()
}

#[allow(unused)]
fn save_classes() {
    _ = std::fs::create_dir("output");

    for file_name in FILES_NAMES {
        println!("Data for class: {file_name}");

        let class_data = class_data(file_name);

        // for item in class_data {
        //     println!("{:#?}", item);
        // }
        let writer = File::create(format! {"output/{file_name}.json"}).expect("File must exist");
        serde_json::to_writer_pretty(writer, &class_data).unwrap();
    }
}

fn class_data(file_name: &str) -> Vec<Section> {
    let section_file =
        File::open(format! {"data/section/{file_name}.json"}).expect("File must exist");
    let reader = BufReader::new(section_file);
    let root: SectionRoot = serde_json::from_reader(reader).unwrap();
    let data = root.data;

    // let data = data.iter().filter(|x| filter_in_person(x));
    // let data: Vec<_> = data.filter(|x| filter_8_30(x)).collect();

    let data: Vec<Section> = data
        .into_iter()
        .filter_map(|data| {
            let meetings = data
                .section_details
                .iter()
                .flat_map(|section| {
                    let meet_days: Vec<Day> = {
                        let mut meet_days = Vec::new();

                        let days = section.meet_days.to_lowercase();

                        if days.contains("mon") {
                            meet_days.push(Day::Mon);
                        }
                        if days.contains("tue") {
                            meet_days.push(Day::Tue);
                        }
                        if days.contains("wed") {
                            meet_days.push(Day::Wed);
                        }
                        if days.contains("thu") {
                            meet_days.push(Day::Thu);
                        }
                        if days.contains("fri") {
                            meet_days.push(Day::Fri);
                        }

                        meet_days
                    };

                    let campus = if section.location.contains("North") {
                        Campus::North
                    } else {
                        Campus::Centinnial
                    };

                    // NOTE: This assumes every class is at the same time
                    let calendar_info = &section.calendar_info[0];

                    let time = TimeRange {
                        start: calendar_info.start_time.parse().unwrap(),
                        end: calendar_info.end_time.parse().unwrap(),
                    };

                    meet_days
                        .iter()
                        .map(move |&meet_day| DayTime {
                            meet_day,
                            time,
                            campus,
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            let professor: String = data
                .section_details
                .iter()
                .find_map(|x| x.instructors.first())
                .unwrap_or(&String::from(""))
                .clone();

            Some(Section {
                class: data.classs,
                section: data.section_details[0].section.clone(),
                facility: data.section_details[0].facility.clone(),
                meetings,
                professor,
            })
        })
        .collect();

    data
}

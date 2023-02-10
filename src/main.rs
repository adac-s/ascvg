use std::{path::PathBuf, fs};

use iced::{Sandbox, widget::{row,column, svg, button, slider, container}, Settings, Length};
use rfd::FileDialog;

mod textgrid;

fn main() {
    let _ = Ascvg::run(Settings::default());
}

enum SaveType {
    Text,
    Svg,
}

#[derive(Debug)]
struct Ascvg {
    path: Option<PathBuf>,
    working_str: Vec<Vec<char>>,

    width: usize,
    height: usize,

    selected: (usize, usize),
}

#[derive(Debug, Clone)]
enum Message {
    New,
    Open,
    Save,
    SaveAs,
    Export,

    ChangeChar(char, (usize, usize)),

    ChangeXSize(usize),
    ChangeYSize(usize),
}

impl Ascvg {
    pub(crate) fn new_file(&mut self) {
        self.selected = (0,0);
        self.working_str = vec![vec![' '; 10]; 10];
    }

    pub(crate) fn query_file(&mut self) {
        let res = FileDialog::new()
            .add_filter("text", &["txt"])
            .pick_file();

        if let Some(p) = res {
            self.selected = (0,0);
            let reader: String = fs::read_to_string(&p).unwrap_or_else(|_| String::new());
            self.height = 0;
            self.width = 0;
            self.working_str = vec![];
            let mut max_w = 0;
            let inter: Vec<Vec<char>> = reader.lines().map(|x| {
                self.height += 1;
                if x.len() > max_w {
                    max_w = x.len();
                }
                x.chars().collect()
            }).collect();
            self.width = max_w;
            for v in inter {
                let mut g = v.clone();
                g.append(&mut vec![' ' ;max_w - v.len()]);
                self.working_str.push(g);
            }
        }
    }

    pub(crate) fn save_file(&mut self) -> eyre::Result<()> {
        let gen_string = self.working_str.iter()
            .fold(String::new(), |acc, l| {
                acc + &l.into_iter().collect::<String>() + "\n" 
            });
        if let Some(p) = &self.path {
            fs::write(p, gen_string)?;
        }
        else {
            self.save_as_intermediate(gen_string, true, SaveType::Text)?;
        }
        Ok(())
    }

    pub(crate) fn save_file_as(&mut self) -> eyre::Result<()> {
        let gen_string = self.working_str.iter()
            .fold(String::new(), |acc, l| {
                acc + &l.into_iter().collect::<String>() + "\n" 
            });
        self.save_as_intermediate(gen_string, true, SaveType::Svg)?;
        Ok(())
    }

    pub(crate) fn export_file(&mut self) -> eyre::Result<()> {
        let gen_string = self.working_str.iter()
            .fold(String::new(), |acc, l| {
                acc + &l.into_iter().collect::<String>() + "\n" 
            });
        let svg_content = svgbob::to_svg_string_pretty(&gen_string);
        self.save_as_intermediate(svg_content, false, SaveType::Svg)?;
        Ok(())
    }

    fn save_as_intermediate(&mut self, content: String, update: bool, str_type: SaveType) -> eyre::Result<()> {
        let (text, filex) = match str_type {
            SaveType::Text => ("text", "txt"),
            SaveType::Svg => ("svg", "svg")
        };

        let res = FileDialog::new()
            .add_filter(text, &[filex])
            .save_file();

     
        if let Some(p) = res {
            fs::write(&p, content)?;
            if update {
                self.path = Some(p);
            }
        }

        Ok(())
    }
}

impl Sandbox for Ascvg {
    type Message = Message;

    fn new() -> Self {
        Self { 
            path: None, 
            working_str: vec![vec![' '; 10]; 10],
            width: 10,
            height: 10, 
            selected: (0, 0)
        }
    }

    fn title(&self) -> String {
        "Ascvg".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::New => self.new_file(),
            Message::Open => self.query_file(),
            Message::Save => self.save_file().unwrap(),
            Message::SaveAs => self.save_file_as().unwrap(),
            Message::Export => self.export_file().unwrap(),
            Message::ChangeChar(ch, (x, y)) => {
                self.working_str[y][x] = ch;
                self.selected = (x, y);
            },
            Message::ChangeXSize(sz) => {
                if sz > self.width {
                    self.working_str = self.working_str.iter().map(|x| {
                        let mut y = x.clone();
                        y.append(&mut vec![' '; sz - self.width]);
                        y
                    }).collect();
                }
                else {
                    self.working_str = self.working_str.iter().map(|x| {
                        x.split_at(sz).0.to_vec()
                    }).collect();
                }
                self.width = sz;
            },
            Message::ChangeYSize(sz) => {
                if sz > self.height {
                    self.working_str.append(&mut vec![vec![' '; self.width]; sz - self.height]);
                }
                else {
                    self.working_str = self.working_str.split_at(sz).0.to_vec();
                }
                self.height = sz;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let gen_string = &self.working_str.iter()
            .fold(String::new(), |acc, l| {
                acc + &l.into_iter().collect::<String>() + "\n" 
            });

        column![
            row![
                button("New").on_press(Message::New),
                button("Open").on_press(Message::Open),
                button("Save").on_press(Message::Save),
                button("SaveAs").on_press(Message::SaveAs),
                button("Export").on_press(Message::Export),
            ],
            row![
                container(
                    textgrid::TextGrid::new(&self.working_str, self.width, self.height, self.selected, |ch, sele| Message::ChangeChar(ch, sele))
                ).width(Length::Fill),
                svg(svg::Handle::from_memory(svgbob::to_svg_string_pretty(
                    gen_string
                ).into_bytes()))
            ],
            slider(1..=100, self.width as i32, |x: i32| Message::ChangeXSize(x.abs() as usize)),
            slider(1..=100, self.height as i32, |x: i32| Message::ChangeYSize(x.abs() as usize)),
        ].spacing(8).padding(8).into()
    }
}

use std::cmp::min;
use iced::{mouse, Event, keyboard};
use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::text;
use iced_native::widget::{self, Widget};
use iced_native::{Color, Element, Length, Point, Rectangle, Size};
pub struct TextGrid<'a, Message> {
    working_str: &'a Vec<Vec<char>>,
    width: usize,
    height: usize,
    selected: (usize, usize), 
    ch_f: Box<dyn Fn(char, (usize, usize)) -> Message + 'a>,
}

impl<'a, Message> TextGrid<'a, Message> {    
    pub fn new<F>(s: &'a Vec<Vec<char>>, w: usize, h: usize, sele: (usize, usize), on_change: F) -> Self 
        where 
            F: 'a + Fn(char, (usize, usize)) -> Message,
    {
        Self { working_str: s, width: w, height: h, selected: sele, ch_f: Box::new(on_change)}
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TextGrid<'a, Message> 
    where 
        Renderer: text::Renderer, 
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(
            (limits.min().width + (self.width as f32 * 20.0)).min(limits.max().width), 
            (limits.min().height + (self.height as f32 * 20.0)).min(limits.max().height - 50.0)
        ))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let (bound_x, bound_y) = (layout.bounds().width, layout.bounds().height);
        let (div_x, div_y) = (bound_x / self.width as f32, bound_y / self.height as f32);
        for i in 0..self.width {
            for j in 0..self.height {
                renderer.fill_text(
                    text::Text { 
                        content: &self.working_str[j][i].to_string(), 
                        bounds: Rectangle { 
                            x: layout.bounds().x + ((i as f32 + 0.5) * div_x), 
                            y: layout.bounds().y + ((j as f32 + 0.5) * div_y), 
                            width: div_x, 
                            height: div_y 
                        },
                        size: 20.0, 
                        color: style.text_color, 
                        font: Default::default(), 
                        horizontal_alignment: iced::alignment::Horizontal::Center, 
                        vertical_alignment: iced::alignment::Vertical::Center 
                    }
                );
                let bg = if (i, j) == self.selected {
                    Color{ r: 0.75, g: 0.75, b: 1.0, a: 1.0 }
                } else {
                    Color::WHITE
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle { 
                            x: layout.bounds().x + (i as f32 * div_x), 
                            y: layout.bounds().y + (j as f32 * div_y), 
                            width: div_x, 
                            height: div_y 
                        },
                        border_radius: 0.0.into(),
                        border_width: 2.0,
                        border_color: Color::BLACK,
                    }, 
                    bg
                );
            }
        }
    }

    fn mouse_interaction(
            &self,
            _state: &widget::Tree,
            layout: Layout<'_>,
            cursor_position: Point,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> iced_native::mouse::Interaction {
        if layout.bounds().contains(cursor_position) {
            mouse::Interaction::Pointer
        }
        else {
            mouse::Interaction::default()
        }
    }

    fn on_event(
            &mut self,
            _state: &mut widget::Tree,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            _renderer: &Renderer,
            _clipboard: &mut dyn iced_native::Clipboard,
            shell: &mut iced_native::Shell<'_, Message>,
        ) -> iced::event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let (bound_x, bound_y) = (layout.bounds().width, layout.bounds().height);
                let (div_x, div_y) = (bound_x / self.width as f32, bound_y / self.height as f32);
                let cpidx = (((cursor_position.x - layout.bounds().x)  / div_x).floor() as usize, ((cursor_position.y - layout.bounds().y) / div_y).floor() as usize);
                self.selected = cpidx;
            },
            Event::Keyboard(keyboard::Event::CharacterReceived(ch)) => {
                let m = (self.ch_f)(ch, self.selected);
                shell.publish(m);
            },
            Event::Keyboard(keyboard::Event::KeyPressed { key_code: keyboard::KeyCode::Up, ..}) => { self.selected.1 = if self.selected.1 == 0 {0} else {self.selected.1 - 1} }
            Event::Keyboard(keyboard::Event::KeyPressed { key_code: keyboard::KeyCode::Down, ..}) => { self.selected.1 = min(self.selected.1 + 1, self.height - 1) }
            Event::Keyboard(keyboard::Event::KeyPressed { key_code: keyboard::KeyCode::Left, ..}) => { self.selected.0 = if self.selected.0 == 0 {0} else {self.selected.0 - 1} }
            Event::Keyboard(keyboard::Event::KeyPressed { key_code: keyboard::KeyCode::Right, ..}) => { self.selected.0 = min(self.selected.0 + 1, self.width - 1) }
            _ => {}
        }
        iced::event::Status::Captured
    }
}

impl<'a, Message: 'a, Renderer> From<TextGrid<'a, Message>> for Element<'a, Message, Renderer> 
    where
        Renderer: text::Renderer {
    fn from(value: TextGrid<'a, Message>) -> Self {
        Self::new(value)
    }
}
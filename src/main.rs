use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyEventKind},
    ExecutableCommand,
};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer, crossterm::event::{self, Event, KeyCode, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Color, Stylize}, symbols::{border, Marker}, text::{Line, Text}, widgets::{
        canvas::{Canvas, Circle, Map, MapResolution, Points, Rectangle}, Block, Paragraph, Widget
    }, DefaultTerminal, Frame
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut appThing = App::default();
    appThing.moves = appThing.get_moves() ;
    let app_result = appThing.run(&mut terminal);
    
    ratatui::restore();
    app_result
}


impl Default for App {
    fn default() -> Self {
        let mut board = [0; 64]; // Initialize all elements to 0
        board[27] = 1;
        board[28] = -1;
        board[36] = 1;
        board[35] = -1;
        
        Self {
            counter: 0,
            player:1,
            exit: false,
            board,
            moves: [].to_vec(), // Manually initialize the array
            
        }
        
    }
}

#[derive(Debug)]
pub struct App {
    counter: i8,
    player: i8,
    exit: bool,
    board: [i8; 64],
    moves: Vec<usize>
    

}


impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        //let vertical = Layout::vertical([Constraint::Percentage(1), Constraint::Percentage(99)]);
        let [left, right] = horizontal.areas(frame.area());
        frame.render_widget(self.map_canvas(), left);
        frame.render_widget(self, right);
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Rothello"))
            .background_color(Color::Green)
            .paint(|ctx| {

                
                

                for i in 0..self.board.len() {
                        let ply = self.board[i];
                        match ply{
                            -1 => ctx.draw(&Circle {
                                x: ((i/8) as f64)*10.0 +5.0,
                                y: ((i%8) as f64)*10.0+5.0,
                                radius: f64::from(3.1),
                                color: Color::White,
                            }),
                            
                            1 => ctx.draw(&Circle {
                                x: ((i/8) as f64)*10.0 +5.0,
                                y: ((i%8) as f64)*10.0+5.0,
                                radius: f64::from(3.1),
                                color: Color::Black,
                            }),
                            
                            _ => (),

                        }
                    
                }

                for i in &self.moves {
                    match self.player{
                        1=>
                    ctx.draw(&Circle {
                        x: ((i/8) as f64)*10.0 +5.0,
                        y: ((i%8) as f64)*10.0+5.0,
                        radius: f64::from(0.55),
                        color: Color::Black,
                    }),
                    -1 => ctx.draw(&Circle {
                        x: ((i/8) as f64)*10.0 +5.0,
                        y: ((i%8) as f64)*10.0+5.0,
                        radius: f64::from(0.55),
                        color: Color::White,
                    }),
                    _ => ()
                }
            }

                ctx.draw(&Circle {
                    x: ((self.counter/8) as f64)*10.0 + 5.0,
                    y: ((self.counter%8) as f64)*10.0 + 5.0,
                    radius: f64::from(0.75),
                    color: Color::LightYellow,
                });

                for i in 0..=8 {
                    for j in 0..=8 {
                        ctx.draw(&Rectangle {
                            x: (i as f64)*10.0,
                            y: (j as f64)*10.0,
                            width: f64::from(10),
                            height: f64::from(10),
                            color: Color::Black,
                        });
                    }
                    
                }
            }   
              
            )
            .x_bounds([0.0, 80.0])
            .y_bounds([0.0, 80.0])
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Enter => self.play(),
            _ => {}
        }
    }

    fn get_moves(&mut self) -> Vec<usize>{
        let vects = [(-8,8),(-1,1),(-7,7),(-9,9)];
        let  mut valid_moves: Vec<usize> = [].to_vec();
        for pos in 0..self.board.len(){
            if self.board[pos] == self.player{
            for v in vects{
                let mut is_valid = false;
                let mut current = pos as i32;
                loop {
                    if (current + v.0) < 0{
                        is_valid = false;
                        break;
                    }
                    if self.board[pos] == -self.board[(current + v.0) as usize]{
                        is_valid = true
                    }
                    if is_valid && self.board[(current + v.0) as usize] == 0{
                        valid_moves.push((current + v.0) as usize);
                        break
                    }
                    current = current+v.0;
                }
                current = pos as i32;
                is_valid = false;
                loop {
                    if (current + v.1) > 63{
                        is_valid = false;
                        break;
                    }
                    if self.board[pos] == -self.board[(current + v.1) as usize]{
                        is_valid = true
                    }
                    if is_valid && self.board[(current + v.1) as usize] == 0{
                        valid_moves.push((current + v.1) as usize);
                        break
                    }
                    current = current+v.1;
                }
            }
        }
        

        }
        //println!("{:?}",valid_moves);
        valid_moves

    }

    fn play(&mut self){
        if self.moves.contains(&(self.counter as usize)){
            self.board[self.counter as usize] = self.player;
            self.player = -self.player;
            self.moves = self.get_moves();
        }
        
        
        


    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_right(&mut self) {
        if self.counter < 48 {
        self.counter += 8;
        }
        else{
            self.counter = self.counter%8;
        }

    }

    fn move_up(&mut self) {
        if self.counter < 63{
            self.counter += 1;
        }
        else{
            self.counter -= 7;
        }
    }

    fn move_down(&mut self) {
        if self.counter > 0 {
        self.counter -= 1;
        }
        else{
            self.counter=63;
        }
    }
    fn move_left(&mut self) {
        if self.counter > 7 {
            self.counter -= 8;
            }
            else{
                self.counter=64-(8-self.counter);
            }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

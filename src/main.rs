use std::io::{self};

use crossterm::event::{ KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer, crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, symbols::{border}, text::{Line, Text}, widgets::{
        canvas::{Canvas, Circle,  Rectangle}, Block, Paragraph, Widget
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
        let mut board = [[0; 8]; 8]; // Initialize all elements to 0
        board[3][3] = 1;
        board[3][4] = -1;
        board[4][3] = -1;
        board[4][4] = 1;
        
        Self {
            counter: [0,0],
            player:1,
            exit: false,
            board,
            scroll: 0,
            moves: [].to_vec(),
            moves_played: [].to_vec(), 
            current_score: (2,2),// Manually initialize the array
            
        }
        
    }
}

#[derive(Debug)]
pub struct App {
    counter: [i8; 2],
    player: i8,
    exit: bool,
    board: [[i8; 8]; 8],
    moves: Vec<(usize,usize)>,
    moves_played: Vec<String>,
    current_score: (i8,i8),
    scroll: u16,
    

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
                    for j in 0..self.board.len() {
                        let ply = self.board[i][j];
                        match ply{
                            -1 => ctx.draw(&Circle {
                                x: (i as f64)*10.0 +5.0,
                                y: (j as f64)*10.0+5.0,
                                radius: f64::from(3.1),
                                color: Color::White,
                            }),
                            
                            1 => ctx.draw(&Circle {
                                x: (i as f64)*10.0 +5.0,
                                y: (j as f64)*10.0+5.0,
                                radius: f64::from(3.1),
                                color: Color::Black,
                            }),
                            
                            _ => (),

                        }
                    }
                    
                }

                for i in &self.moves {
                    match self.player{
                        1=>
                    ctx.draw(&Circle {
                        x: (i.0 as f64)*10.0 +5.0,
                        y: (i.1 as f64)*10.0+5.0,
                        radius: f64::from(0.55),
                        color: Color::Black,
                    }),
                    -1 => ctx.draw(&Circle {
                        x: (i.0 as f64)*10.0 +5.0,
                        y: (i.1 as f64)*10.0+5.0,
                        radius: f64::from(0.55),
                        color: Color::White,
                    }),
                    _ => ()
                }
            }

                ctx.draw(&Circle {
                    x: ((self.counter[0]) as f64)*10.0 + 5.0,
                    y: ((self.counter[1]) as f64)*10.0 + 5.0,
                    radius: f64::from(0.75),
                    color: Color::Rgb((244), (214), (255)),
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
            KeyCode::Char('w') => self.scroll_up(),
            KeyCode::Char('s') => self.scroll_down(),
            _ => {}
        }
    }

    fn get_moves(&mut self) -> Vec<(usize,usize)>{
        let vects: [(i8, i8); 8] = [(0,1),(1,0),(0,-1),(-1,0),(-1,1),(1,-1),(1,1),(-1,-1)];
        let  mut valid_moves: Vec<(usize,usize)> = [].to_vec();
        for x in 0..self.board.len(){
            for y in 0..self.board.len(){
                if self.board[x][y] == self.player{
                for v in vects{
                    let mut is_valid = false;
                    let mut current = (x as i8,y as i8);
                    loop {
                        if (current.0 + v.0) < 0 || (current.1 + v.1) < 0 || (current.0 + v.0) > 7 || (current.1 + v.1) > 7 {
                            is_valid = false;
                            break;
                        }
                        if self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize] == self.player{
                            break;
                        }
                        if self.board[x][y] == -self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize]{
                            is_valid = true
                        }
                        if is_valid && self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize]  == 0{
                            valid_moves.push(((current.0 + v.0) as usize,(current.1 + v.1) as usize));
                            break
                        }
                        if self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize]  == 0{
                            break;
                        }
                        current = (current.0+v.0,current.1+v.1);
                    }
                }
            }
        }
        

        }
        //println!("{:?}",valid_moves);
        valid_moves

    }

    fn flip_chips(&mut self,pos:(usize,usize)){
        let vects: [(i8, i8); 8] = [(0,1),(1,0),(0,-1),(-1,0),(-1,1),(1,-1),(1,1),(-1,-1)];
        let mut to_flip: Vec<(usize,usize)> = [].to_vec();
        match self.player{
            -1=> self.current_score = (self.current_score.0+1,self.current_score.1),
            1=>self.current_score = (self.current_score.0+0,self.current_score.1+1),
            _ => ()
        }
        for v in vects{
            let mut current = (pos.0 as i8,pos.1 as i8);
            to_flip = [].to_vec();
            loop {
                if (current.0 + v.0) < 0 || (current.1 + v.1) < 0 || (current.0 + v.0) > 7 || (current.1 + v.1) > 7 {
                    break;
                }
                if self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize] == 0{
                    break;
                }
                if self.board[pos.0 as usize][pos.1 as usize] == -self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize]{
                    to_flip.push(((current.0 + v.0) as usize,(current.1 + v.1) as usize))
                }
                if self.board[(current.0 + v.0) as usize][(current.1 + v.1) as usize] == self.board[pos.0 as usize][pos.1 as usize]{
                    match self.player{
                        -1=> self.current_score = (self.current_score.0+to_flip.len() as i8,self.current_score.1-to_flip.len() as i8),
                        1=>self.current_score = (self.current_score.0-to_flip.len() as i8,self.current_score.1+to_flip.len() as i8),
                        _ => ()
                    }
                    for i in to_flip{
                        self.board[i.0][i.1] = -self.board[i.0][i.1];
                    }

                    


                    break
                }
                current = (current.0+v.0,current.1+v.1);
            }
            
            
        }
            
    }

    fn play(&mut self){
        //println!("{:?}",self.moves);
        let current_move = (self.counter[0] as usize, self.counter[1] as usize);
        if self.moves.contains(&(current_move)){
            let playername;
            match self.player{
                -1 =>  playername = "White",
                1 => playername = "Black",
                _ => playername = "Unknown",
            }
            let snapshot = self.current_score;
            self.board[current_move.0][current_move.1] = self.player;
            self.flip_chips(current_move);
            self.player = -self.player;
            self.moves = self.get_moves();
            let snapshot2 = (self.current_score.0 - snapshot.0,self.current_score.1 - snapshot.1);
            self.moves_played.push(format!("{} - played at x={} y= {} - Points W:{} || B:{} \n", playername,current_move.0,current_move.1,snapshot2.0,snapshot2.1));
            
        }
        
        
        


    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_right(&mut self) {
        if self.counter[0] < 7{
            self.counter[0] += 1
        }
        else{
            self.counter[0] = 0;
        }

    }

    fn move_up(&mut self) {
        if self.counter[1] < 7{
            self.counter[1] += 1
        }
        else{
            self.counter[1] = 0;
        }
    }

    fn move_down(&mut self) {
        if self.counter[1] > 0{
            self.counter[1] += -1
        }
        else{
            self.counter[1] = 7;
        }
    }
    fn move_left(&mut self) {
        if self.counter[0] > 0{
            self.counter[0] += -1
        }
        else{
            self.counter[0] = 7;
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll != 0{
            self.scroll = (self.scroll - 1) % (self.moves_played.len() as u16 +1);
        }
    }
    fn scroll_down(&mut self) {
        self.scroll = (self.scroll + 1) % (self.moves_played.len() as u16 +1);
    }    
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Welcome to Roth! ".bold());
        let instructions = Line::from(vec![
            " Move ".into(),
            "< ← ↑ ↓ → >".blue().bold(),
            " Play ".into(),
            "<Enter>".blue().bold(),
            " Scroll ".into(),
            "<W/S>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut display_vect = vec![Line::from(vec![
            "Current position: x=".into(),
            self.counter[0].to_string().yellow(),
            " - y=".into(),
            self.counter[1].to_string().yellow(),
        ]),
        Line::raw(format!("White - {} ||  Black - {}",self.current_score.0,self.current_score.1))
        
        ];
        for i in &self.moves_played{
            display_vect.push(Line::from(i as &str));
        }
        let counter_text = Text::from(display_vect);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}

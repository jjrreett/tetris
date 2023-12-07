use crossterm::terminal;

use std::io::Result;

use ratatui::prelude as tui;
use ratatui::widgets;

use itertools::Itertools;

mod utils;
use utils::Vec2;

mod tetromino;
use tetromino::Colors;
use tetromino::MoveDirection;
use tetromino::Tetromino;
use tetromino::TetrominoRotation;
use tetromino::TetrominoShape;

mod space_partition;

type Terminal = tui::Terminal<tui::CrosstermBackend<std::io::Stdout>>;

// fn gen_2d_range(from: usize, to: usize) -> impl Iterator<Item = (usize, usize)> {
//     (from..to).flat_map(move |a| (from..to).map(move |b| (a, b)))
// }

const TILE_SIZE: Vec2<usize> = Vec2 { x: 3, y: 2 };
const TILE_CHARS: [[char; TILE_SIZE.x]; TILE_SIZE.y] = [['┌', '─', '┐'], ['└', '─', '┘']];
// const TILE_CHARS: [[char; TILE_SIZE.x]; TILE_SIZE.y] = [['▛', '▀', '▜'], ['▙', '▅', '▟']];
// const TILE_CHARS: [[char; TILE_SIZE.x]; TILE_SIZE.y] = [['▉', '▉', '▉'], ['▉', '▉', '▉']];

const GAME_SIZE: Vec2<usize> = Vec2 { x: 10, y: 22 };

#[derive(Clone, Copy)]
struct ActivePiece {
    pos: Vec2<usize>,
    tetromino: Tetromino,
}

#[derive(Clone, Copy)]
struct GameBoard {
    grid: [[Option<Colors>; GAME_SIZE.x]; GAME_SIZE.y],
    grid_dims_chars: Vec2<usize>,
    active_peice: Option<ActivePiece>,
}
impl GameBoard {
    fn new() -> Self {
        Self {
            grid: [[None; GAME_SIZE.x]; GAME_SIZE.y],
            grid_dims_chars: Vec2::<usize>::new(
                TILE_SIZE.x * GAME_SIZE.x,
                TILE_SIZE.y * GAME_SIZE.y,
            ),
            active_peice: None,
        }
    }
    fn grid_tile_buf_iterator(
        &self,
        area: tui::Rect,
    ) -> impl Iterator<Item = ((usize, usize), (usize, usize), (usize, usize), (u16, u16))> + '_
    {
        (0..self.grid_dims_chars.y).flat_map(move |char_y| {
            (0..self.grid_dims_chars.x).map(move |char_x| {
                (
                    (char_y / TILE_SIZE.y, char_x / TILE_SIZE.x),
                    (char_y, char_x),
                    (char_y % TILE_SIZE.y, char_x % TILE_SIZE.x),
                    (area.top() + char_y as u16, area.left() + char_x as u16),
                )
            })
        })
    }
    fn draw_to_board(&mut self, pos: Vec2<usize>, tetromino: Tetromino) {
        for (_grid_y, _grid_x) in tetromino.blocks.iter() {
            let (grid_y, grid_x) = (_grid_y + pos.y, _grid_x + pos.x);
            self.grid[grid_y][grid_x] = Some(tetromino.color);
        }
    }
}

impl<'a> widgets::Widget for GameBoard {
    fn render(self, area: tui::Rect, buf: &mut tui::Buffer) {
        for ((grid_y, grid_x), (char_y, char_x), (tile_y, tile_x), (buf_y, buf_x)) in
            self.grid_tile_buf_iterator(area)
        {
            match self.grid[grid_y][grid_x] {
                Some(monomino) => {
                    let character = TILE_CHARS[tile_y][tile_x];
                    let color = monomino.to_tui_color();
                    buf.get_mut(buf_x, buf_y)
                        .set_symbol(&character.to_string())
                        .set_fg(color);
                }
                None => {}
            };
        }
        if let Some(ap) = self.active_peice {
            for (grid_y, grid_x) in ap.tetromino.blocks.iter() {
                for (tile_y, tile_x) in (0..TILE_SIZE.y).cartesian_product(0..TILE_SIZE.x) {
                    let (pos_on_grid_y, pos_on_grid_x) = (grid_y + ap.pos.y, grid_x + ap.pos.x);
                    let (char_y, char_x) =
                        (pos_on_grid_y * TILE_SIZE.y, pos_on_grid_x * TILE_SIZE.x);
                    let (buf_y, buf_x) = (
                        area.top() + (char_y + tile_y) as u16,
                        area.left() + (char_x + tile_x) as u16,
                    );

                    let character = TILE_CHARS[tile_y][tile_x];

                    buf.get_mut(buf_x, buf_y)
                        .set_symbol(&character.to_string())
                        .set_fg(ap.tetromino.color.to_tui_color());
                }
            }
        }
    }
}

enum Action {
    Quit,
    DebugDrawCurrentPiece,
    MovePiece(MoveDirection),
}

struct App {
    game_board: GameBoard,
    should_quit: bool,
    debug_text: String,
}

impl App {
    fn new() -> App {
        App {
            game_board: GameBoard::new(),
            should_quit: false,
            debug_text: String::from("Hello Wold\n"),
        }
    }

    fn get_user_input(&self) -> Result<Option<Action>> {
        use crossterm::event as c_event;

        let timeout = std::time::Duration::from_millis(100);
        if c_event::poll(timeout)? {
            if let c_event::Event::Key(key) = c_event::read()? {
                if key.kind == c_event::KeyEventKind::Press {
                    return Ok(match key.code {
                        c_event::KeyCode::Char('q') => Some(Action::Quit),
                        c_event::KeyCode::Char('d') => Some(Action::DebugDrawCurrentPiece),
                        c_event::KeyCode::Char('h') => Some(Action::MovePiece(MoveDirection::Left)),
                        c_event::KeyCode::Char('j') => Some(Action::MovePiece(MoveDirection::Down)),
                        c_event::KeyCode::Char('l') => {
                            Some(Action::MovePiece(MoveDirection::Right))
                        }
                        c_event::KeyCode::Char('H') => Some(Action::MovePiece(MoveDirection::CCW)),
                        c_event::KeyCode::Char('L') => Some(Action::MovePiece(MoveDirection::CW)),
                        _ => None,
                    });
                }
            }
        }
        Ok(None) // Return Ok(None) if no event is detected or if the event is not a key press
    }

    // Logic for making the current piece static and making a new one
    // {
    //     if let Some(ap) = self.game_board.active_peice {
    //         self.game_board.draw_to_board(ap.pos, ap.tetromino);
    //         let new_ap = ActivePiece {
    //             pos: Vec2::new(3, 0),
    //             tetromino: TetrominoShape::get_data(
    //                 TetrominoShape::O,
    //                 TetrominoRotation::Zero,
    //             ),
    //         };
    //         self.game_board.active_peice = Some(new_ap);
    //     }
    // }

    fn game_loop(&mut self, mut terminal: Terminal) -> Result<()> {
        let input_rate = std::time::Duration::from_millis(100);
        let loop_rate = std::time::Duration::from_millis(10); // consistent loop rate
        let gravity_frame_rate = std::time::Duration::from_millis(1000);

        let mut last_input_check = std::time::Instant::now();
        let mut last_gravity_frame_update = std::time::Instant::now();
        let mut needs_redraw = true; // flag to track if we need to redraw

        loop {
            let loop_start = std::time::Instant::now();
            // Input handling
            if last_input_check.elapsed() >= input_rate {
                if let Some(action) = self.get_user_input()? {
                    match action {
                        Action::Quit => self.should_quit = true,
                        Action::DebugDrawCurrentPiece => {
                            if let Some(ap) = self.game_board.active_peice {
                                self.game_board.draw_to_board(ap.pos, ap.tetromino);
                                let new_ap = ActivePiece {
                                    pos: Vec2::new(3, 0),
                                    tetromino: Tetromino::new(
                                        TetrominoShape::pick_random_shape(),
                                        TetrominoRotation::Zero,
                                    ),
                                };
                                self.game_board.active_peice = Some(new_ap);
                            }
                        }
                        Action::MovePiece(direction) => {
                            match direction {
                                MoveDirection::Left => {
                                    if let Some(ap) = self.game_board.active_peice {
                                        self.game_board.active_peice = Some(ActivePiece {
                                            pos: Vec2::new(ap.pos.x - 1, ap.pos.y),
                                            tetromino: ap.tetromino,
                                        });
                                    }
                                }
                                MoveDirection::Right => {
                                    if let Some(ap) = self.game_board.active_peice {
                                        self.game_board.active_peice = Some(ActivePiece {
                                            pos: Vec2::new(ap.pos.x + 1, ap.pos.y),
                                            tetromino: ap.tetromino,
                                        });
                                    }
                                }
                                MoveDirection::Down => {
                                    if let Some(ap) = self.game_board.active_peice {
                                        last_gravity_frame_update = std::time::Instant::now();
                                        self.game_board.active_peice = Some(ActivePiece {
                                            pos: Vec2::new(ap.pos.x, ap.pos.y + 1),
                                            tetromino: ap.tetromino,
                                        });
                                    }
                                }
                                MoveDirection::CCW => {
                                    if let Some(ap) = self.game_board.active_peice {
                                        let new_tetromino = ap.tetromino.rotate(MoveDirection::CCW);
                                        let new_ap = ActivePiece {
                                            pos: ap.pos,
                                            tetromino: new_tetromino,
                                        };
                                        self.game_board.active_peice = Some(new_ap);
                                    }
                                }
                                MoveDirection::CW => {
                                    if let Some(ap) = self.game_board.active_peice {
                                        let new_tetromino = ap.tetromino.rotate(MoveDirection::CW);
                                        let new_ap = ActivePiece {
                                            pos: ap.pos,
                                            tetromino: new_tetromino,
                                        };
                                        self.game_board.active_peice = Some(new_ap);
                                    }
                                }
                                MoveDirection::FirmDrop => todo!(),
                            }
                            needs_redraw = true;
                        }
                    };
                    last_input_check = std::time::Instant::now();
                }
            }

            // If a move action occured, recompute ghost block

            // Gravity handling
            if last_gravity_frame_update.elapsed() >= gravity_frame_rate {
                if let Some(ap) = self.game_board.active_peice {
                    let new_ap = ActivePiece {
                        pos: ap.pos + Vec2::new(0, 1),
                        tetromino: ap.tetromino,
                    };
                    // check if new ap is valid location
                    // else draw to board, istantiate new peice
                    self.game_board.active_peice = Some(new_ap);
                }

                last_gravity_frame_update = std::time::Instant::now();
                needs_redraw = true;
            }

            // Redraw frame if needed
            if needs_redraw {
                terminal.draw(|frame: &mut tui::Frame<'_>| self.make_frame(frame))?;
                needs_redraw = false; // reset flag after redrawing
            }

            // Check for quit condition
            if self.should_quit {
                break;
            }

            let time_now = std::time::Instant::now();
            let loop_duration_already_served = time_now.duration_since(loop_start);
            if let Some(sleep_time) = loop_rate.checked_sub(loop_duration_already_served) {
                std::thread::sleep(sleep_time);
            }
        }
        Ok(())
    }

    fn make_frame(&self, frame: &mut tui::Frame) {
        const MIN_SIZE: Vec2<u16> = Vec2 { x: 60, y: 48 };

        let frame_area = frame.size();

        if frame_area.height < MIN_SIZE.y || frame_area.width < MIN_SIZE.x {
            let size_warning = format!(
                "Required Terminal Size is {}x{}\nYour Terminal Size is {}x{}",
                MIN_SIZE.x, MIN_SIZE.y, frame_area.width, frame_area.height
            );
            frame.render_widget(
                widgets::Paragraph::new(size_warning).block(
                    widgets::Block::default()
                        .title("Error")
                        .borders(widgets::Borders::ALL),
                ),
                frame_area,
            );
            // TODO figure out how to get the app to not continue from here
            return;
        }

        let top_level_layout = tui::Layout::default()
            .direction(tui::Direction::Vertical)
            .constraints([
                tui::Constraint::Length(self.game_board.grid_dims_chars.y as u16),
                tui::Constraint::Min(10),
            ])
            .split(frame_area);

        let debug_layout = top_level_layout[1];
        let game_layout = top_level_layout[0];

        let wing_width = (game_layout.width - self.game_board.grid_dims_chars.x as u16) / 2;

        let game_layout = tui::Layout::default()
            .direction(tui::Direction::Horizontal)
            .constraints([
                tui::Constraint::Length(wing_width),
                tui::Constraint::Length(self.game_board.grid_dims_chars.x as u16),
                tui::Constraint::Length(wing_width),
            ])
            .split(top_level_layout[0]);

        let left = game_layout[0];
        let board_layout = game_layout[1];
        let right = game_layout[2];

        frame.render_widget(
            widgets::Paragraph::new(&*self.debug_text).block(
                widgets::Block::default()
                    .title("debug")
                    .borders(widgets::Borders::ALL),
            ),
            debug_layout,
        );
        frame.render_widget(
            widgets::Paragraph::new("this is the left block").block(
                widgets::Block::default()
                    .title("left")
                    .borders(widgets::Borders::ALL),
            ),
            left,
        );
        frame.render_widget(
            widgets::Paragraph::new("this is the right block").block(
                widgets::Block::default()
                    .title("right")
                    .borders(widgets::Borders::ALL),
            ),
            right,
        );
        frame.render_widget(self.game_board, board_layout);
    }
}

fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn main() -> Result<()> {
    initialize_panic_handler();

    terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), terminal::EnterAlternateScreen)?;

    let mut terminal = tui::Terminal::new(tui::CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    // let mut t = TetrominoShape::get_data(TetrominoShape::L, TetrominoRotation::Zero);
    // t.move_down(5);

    app.game_board.active_peice = Some(ActivePiece {
        pos: Vec2::new(3, 0),
        tetromino: Tetromino::new(TetrominoShape::pick_random_shape(), TetrominoRotation::Zero),
    });

    let status = app.game_loop(terminal);

    crossterm::execute!(std::io::stderr(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    status?;
    Ok(())
}

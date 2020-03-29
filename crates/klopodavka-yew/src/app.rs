use klopodavka_lib::game::{GameState, HeatMapTile};
use klopodavka_lib::models::*;
use klopodavka_lib::{ai, game};
use std::time::Duration;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

pub struct App {
    new_game_click: Callback<ClickEvent>,
    cell_click: Vec<Vec<Callback<ClickEvent>>>,
    game: GameState,
    console: ConsoleService,

    #[allow(dead_code)]
    callback_tick: Callback<()>,

    #[allow(dead_code)]
    tick_handle: Box<dyn Task>,
}

pub enum Msg {
    MakeMove(Pos),
    NewGame,
    Tick,
}

fn heat_map_color(heat: u8, max_heat: u8) -> u8 {
    let base_color: u8 = 230;
    let max_color: u8 = 250;
    let color_range = (max_color - base_color) as f32;

    max_color - (heat as f32 / max_heat as f32 * color_range) as u8
}

fn render_tile(app: &App, pos: Pos) -> Html {
    let tile = app.game.tile(pos);

    // TODO: Render disconnected tiles in a different way.
    let (mut text, mut style) = match tile {
        Tile::Empty => {
            let heat = app.game.heat(pos);
            if heat != (HeatMapTile { red: 0, blue: 0 }) {
                let red = heat_map_color(heat.red, app.game.max_heat());
                let blue = heat_map_color(heat.blue, app.game.max_heat());
                let rgb = format!("background-color: rgb({}, 250, {})", blue, red);

                ("", rgb)
            } else {
                ("", "".to_string())
            }
        }
        Tile::Base(Player::Red) => ("🏠", "background-color: #ff9999".to_string()),
        Tile::Base(Player::Blue) => ("🏠", "background-color: #80b3ff".to_string()),
        Tile::Alive(Player::Red) => ("", "background-color: #ff9999".to_string()),
        Tile::Alive(Player::Blue) => ("", "background-color: #80b3ff".to_string()),
        Tile::Squashed(Player::Red) => ("", "background-color: #cc0000".to_string()),
        Tile::Squashed(Player::Blue) => ("", "background-color: #005ce6".to_string()),
    };

    if app.game.is_valid_move(pos) {
        text = "·";

        style.push_str("; cursor: pointer");

        render_tile_avail(text, style.as_str(), app, pos)
    } else {
        render_tile_nonavail(text, style.as_str())
    }
}

fn render_tile_avail(text: &str, style: &str, app: &App, pos: Pos) -> Html {
    let click_handler = app
        .cell_click
        .get(pos.x as usize)
        .expect("click handler column")
        .get(pos.y as usize)
        .expect("click handler");

    html! {
        <td style=style onclick=click_handler>{ text }</td>
    }
}

fn render_tile_nonavail(text: &str, style: &str) -> Html {
    html! {
        <td style=style>{ text }</td>
    }
}

fn render_row(app: &App, y: u16) -> Html {
    html! {
        <tr>
            { (0.. BOARD_WIDTH).map(|x| render_tile(app, Pos {x, y})).collect::<Html>() }
        </tr>
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let callback_tick = link.callback(|_| Msg::Tick);
        let handle = interval.spawn(Duration::from_millis(150), callback_tick.clone());
        let tick_handle = Box::new(handle);

        App {
            game: game::GameState::new(),
            cell_click: (0..BOARD_WIDTH)
                .map(|x| {
                    (0..BOARD_HEIGHT)
                        .map(|y| link.callback(move |_| Msg::MakeMove(Pos { x, y })))
                        .collect()
                })
                .collect(),
            new_game_click: link.callback(|_| Msg::NewGame),
            console: ConsoleService::new(),
            callback_tick,
            tick_handle,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let game_state = &mut self.game;

        match msg {
            Msg::MakeMove(pos) => {
                if game_state.current_player() == Player::Red {
                    game_state.make_move(pos);
                    true
                } else {
                    false
                }
            }

            Msg::NewGame => {
                self.console.info("New game");
                self.game = game::GameState::new();
                true
            }

            Msg::Tick => {
                if game_state.current_player() == Player::Blue {
                    if let Some(m) = ai::get_ai_move(game_state) {
                        game_state.make_move(m.pos);
                        return true;
                    }
                }

                false
            }
        }
    }

    fn view(&self) -> Html {
        let g = &self.game;

        let status = if let Some(winner) = g.winner() {
            format!("Game over, {:?} won!", winner)
        } else {
            format!(
                "Player: {:?} | Clicks: {}",
                g.current_player(),
                g.moves_left()
            )
        };

        html! {
            <>
                <div>
                    <div style="float: right">
                        <img src="rust_logo.svg" alt="Rust Logo" style="width: 80px"/>
                    </div>
                    <img src="logo.svg" alt="Klopodavka Logo" style="width: 640px"/>
                </div>

                <div>
                    <div style="float: right">
                        <button class="button" onclick=&self.new_game_click>{ "New Game" }</button>
                    </div>

                    <div>
                        <strong>{ status }</strong>
                    </div>

                    <p>
                        <table>
                            { (0.. BOARD_HEIGHT).map(|y| render_row(&self, y)).collect::<Html>() }
                        </table>
                    </p>
                </div>

                <div>
                    <span>{"Source: "}</span><a href="https://github.com/ptupitsyn/klopodavka-rs"> { "github.com/ptupitsyn/klopodavka-rs" } </a>
                </div>
            </>
        }
    }
}

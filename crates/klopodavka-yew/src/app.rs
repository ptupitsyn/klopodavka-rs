use klopodavka_lib::game::{GameState, HeatMapTile};
use klopodavka_lib::models::*;
use klopodavka_lib::{ai, game};
use yew::prelude::*;

pub struct App {
    ai_move_click: Callback<ClickEvent>,
    new_game_click: Callback<ClickEvent>,
    cell_click: Vec<Vec<Callback<ClickEvent>>>,
    game: GameState,
}

pub enum Msg {
    MakeAiMove,
    MakeMove(Pos),
    NewGame,
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
        .unwrap()
        .get(pos.y as usize)
        .unwrap();

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
        App {
            ai_move_click: link.callback(|_| Msg::MakeAiMove),
            game: game::GameState::new(),
            cell_click: (0..BOARD_WIDTH)
                .map(|x| {
                    (0..BOARD_HEIGHT)
                        .map(|y| link.callback(move |_| Msg::MakeMove(Pos { x, y })))
                        .collect()
                })
                .collect(),
            new_game_click: link.callback(|_| Msg::NewGame),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let game_state = &mut self.game;

        match msg {
            Msg::MakeAiMove => match ai::get_ai_move(game_state) {
                Some(tile) => {
                    game_state.make_move(tile.pos);
                    true
                }
                None => false,
            },
            Msg::MakeMove(pos) => {
                game_state.make_move(pos);

                // Perform AI moves if current player is AI (Blue).
                // TODO: Perform AI moves on a timer instead, for "animated" look.
                while game_state.current_player() == Player::Blue {
                    match ai::get_ai_move(game_state) {
                        Some(tile) => {
                            game_state.make_move(tile.pos);
                        }
                        None => break,
                    }
                }

                true
            }
            Msg::NewGame => {
                self.game = game::GameState::new();
                true
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
                <img src="logo.svg" alt="Logo" style="width: 640px"/>

                <div>
                    <div style="float: right">
                        <button class="button" style="margin-right: 10px" onclick=&self.ai_move_click>{ "AI Move" }</button>
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

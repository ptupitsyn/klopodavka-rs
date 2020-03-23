use klopodavka_lib::game::GameState;
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

fn render_tile(app: &App, pos: Pos) -> Html {
    let tile = app.game.tile(pos);

    let (mut text, style) = match tile {
        Tile::Empty => ("", ""),
        Tile::Base(Player::Red) => ("🏠", "background-color: #ff9999"),
        Tile::Base(Player::Blue) => ("🏠", "background-color: #80b3ff"),
        Tile::Alive(Player::Red) => ("", "background-color: #ff9999"),
        Tile::Alive(Player::Blue) => ("", "background-color: #80b3ff"),
        Tile::Squashed(Player::Red) => ("", "background-color: #cc0000"),
        Tile::Squashed(Player::Blue) => ("", "background-color: #005ce6"),
    };

    if app.game.is_valid_move(pos) {
        text = "·";

        let mut style = style.to_string();
        style.push_str("; cursor: pointer");

        render_tile_avail(text, style.as_str(), app, pos)
    } else {
        render_tile_nonavail(text, style)
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
                <img src="https://raw.githubusercontent.com/ptupitsyn/klopodavka/master/website/pic/klopodavka.jpg" alt="Logo" style="width: 640px"/>

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

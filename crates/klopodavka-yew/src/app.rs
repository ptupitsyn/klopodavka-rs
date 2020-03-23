use klopodavka_lib::game::GameState;
use klopodavka_lib::models::*;
use klopodavka_lib::{ai, game};
use yew::prelude::*;

pub struct App {
    ai_move_click: Callback<ClickEvent>,
    game: GameState,
}

pub enum Msg {
    MakeAiMove,
}

fn render_tile(game: &GameState, pos: Pos) -> Html {
    let tile = game.tile(pos);

    let (mut text, style) = match tile {
        Tile::Empty => ("", ""),
        Tile::Base(Player::Red) => ("🏠", "background-color: #ff9999"),
        Tile::Base(Player::Blue) => ("🏠", "background-color: #80b3ff"),
        Tile::Alive(Player::Red) => ("", "background-color: #ff9999"),
        Tile::Alive(Player::Blue) => ("", "background-color: #80b3ff"),
        Tile::Squashed(Player::Red) => ("", "background-color: #cc0000"),
        Tile::Squashed(Player::Blue) => ("", "background-color: #005ce6"),
    };

    // TODO: Inefficient check, use a two-dim array instead for O(1) check
    if game.moves().contains(&pos) {
        text = "·";

        let mut style = style.to_string();
        style.push_str("; cursor: pointer");

        render_tile_avail(text, style.as_str())
    } else {
        render_tile_nonavail(text, style)
    }
}

fn render_tile_avail(text: &str, style: &str) -> Html {
    html! {
        <td style=style>{ text }</td>
    }
}

fn render_tile_nonavail(text: &str, style: &str) -> Html {
    html! {
        <td style=style>{ text }</td>
    }
}

fn render_row(game: &GameState, y: u16) -> Html {
    html! {
        <tr>
            { (0.. BOARD_WIDTH).map(|x| render_tile(game, Pos {x, y})).collect::<Html>() }
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
        }
    }

    fn view(&self) -> Html {
        let status = format!(
            "Player: {:?}, Moves: {}",
            &self.game.current_player(),
            &self.game.moves_left()
        );

        html! {
            <div>
                <h1>{ "Klopodavka" }</h1>
                <h3>{ status }</h3>
                <p><button onclick=&self.ai_move_click>{ "AI Move" }</button></p>
                <table>
                    { (0.. BOARD_HEIGHT).map(|y| render_row(&self.game, y)).collect::<Html>() }
                </table>
            </div>
        }
    }
}

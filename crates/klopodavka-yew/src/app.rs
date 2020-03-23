use klopodavka_lib::game::GameState;
use klopodavka_lib::{ai, game};
use yew::prelude::*;

pub struct App {
    make_move_click: Callback<ClickEvent>,
    game: GameState,
}

pub enum Msg {
    MakeMove,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            make_move_click: link.callback(|_| Msg::MakeMove),
            game: game::GameState::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let game_state = &mut self.game;

        match msg {
            Msg::MakeMove => match ai::get_ai_move(game_state) {
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
                <h3> { status } </h3>
                <button onclick=&self.make_move_click>{ "Make a move" }</button>
            </div>
        }
    }
}

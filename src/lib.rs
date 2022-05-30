/*
    *** Tutorial Source - https://www.youtube.com/watch?v=iR7Q_6quwSI&list=PLtTT8p-gjGEdGzZ0ET2bwNnA6iP_mmmrv&index=1 

    TODO:
    - Improve and separate food generation algorithm
    - On html page, win or lose message, score display, game restart 
*/

mod random;
mod snake;

use std::{rc::Rc, cell::RefCell};
use snake::{SnakeGame, Direction};
use wasm_bindgen::{prelude::*, UnwrapThrowExt, JsCast};
use web_sys::{window, console, HtmlElement, HtmlDivElement, KeyboardEvent};
use js_sys::Function;

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(15, 15)));

    static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        GAME.with(|game| game.borrow_mut().tick());
        render();
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = 
        Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {
            let direction = match &evt.key()[..] {
                "ArrowUp" => Some(Direction::UP),
                "ArrowDown" => Some(Direction::DOWN),
                "ArrowLeft" => Some(Direction::LEFT),
                "ArrowRight" => Some(Direction::RIGHT),
                _ => None,
            };

            if let Some(direction) = direction {
                game.borrow_mut().change_direction(direction)
            }
        })) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting...".into());

    HANDLE_TICK.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(), 200
            ).unwrap_throw();
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window().unwrap_throw()
        .add_event_listener_with_callback(
            "keydown",
            handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
        )
        .unwrap_throw()
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let game = game.borrow();
        let document = window().unwrap_throw().document().unwrap_throw();
        let root_container = document
            .get_element_by_id("root").unwrap_throw()
            .dyn_into::<HtmlElement>().unwrap_throw();

        root_container.set_inner_html("");

        let width = game.width;
        let height = game.height;

        root_container
            .style().set_property("display", "inline-grid").unwrap_throw();

        root_container
            .style().set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", height, width)
            ).unwrap_throw();

        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let field_element = document
                    .create_element("div").unwrap_throw()
                    .dyn_into::<HtmlDivElement>().unwrap_throw();

                field_element.set_class_name("field");

                field_element.set_inner_text({
                    if pos == game.food {
                        "🟠"
                    } else if game.snake.get(0) == Some(&pos) {
                        "🟧"
                    } else if game.snake.contains(&pos) {
                        "🟩"
                    } else {
                        "⬜"
                    }
                });

                root_container.append_child(&field_element).unwrap_throw();
            }
        }
    });
}
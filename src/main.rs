use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

impl State {
    fn new() -> Self {
        State {}
    }
}

fn main() -> BError {
    println!(
        r#"
        _____           _           _____                        _ _ _        
        |  __ \         | |         |  __ \                      | (_) |       
        | |__) |   _ ___| |_ _   _  | |__) |___   __ _ _   _  ___| |_| | _____ 
        |  _  / | | / __| __| | | | |  _  // _ \ / _` | | | |/ _ \ | | |/ / _ \
        | | \ \ |_| \__ \ |_| |_| | | | \ \ (_) | (_| | |_| |  __/ | |   <  __/
        |_|  \_\__,_|___/\__|\__, | |_|  \_\___/ \__, |\__,_|\___|_|_|_|\_\___|
                              __/ |               __/ |                        
                             |___/               |___/                         
       
      2022 Axel Prieto
    "#
    );

    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal")
        .build()?;

    main_loop(context, State::new())
}

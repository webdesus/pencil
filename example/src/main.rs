extern crate pencil;

use pencil::Pencil;
use pencil::PenValue;

fn main() {
    let mut app = Pencil::new("/web/example");
    app.route(r"/user/(\d+)", &["GET"], "get_user", PenValue("fengsp".to_string()));
    app.run();
}

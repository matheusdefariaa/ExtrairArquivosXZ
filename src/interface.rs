use cursive::views::Dialog;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::SelectView;

fn main() {
    let mut interface = cursive::default();
    menu_inicial(&mut interface);
    interface.run();
}

fn menu_inicial(i: &mut Cursive) {
    i.pop_layer();
    i.add_layer(Dialog::text("O que deseja fazer ?")
                        .title("Rzip")
                        .button("Listar",listar)
                        .button("Extrair",extrair)
                        .button("Sair",|i| i.quit()));
}

fn listar(i: &mut Cursive) {
    todo!()
}

fn extrair(i: &mut Cursive) {
    todo!();
}
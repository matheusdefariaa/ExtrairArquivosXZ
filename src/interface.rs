pub mod interface {

    use cursive::views::Dialog;
    use cursive::Cursive;
    use cursive::traits::*;
    use cursive::views::SelectView;
    use cursive::align::HAlign;
    use std::fs;

    pub fn interface_main() {
        let mut interface = cursive::default();
        menu_inicial(&mut interface);
        interface.run();
    }

    pub fn menu_inicial(i: &mut Cursive) {
        i.pop_layer();
        i.add_layer(Dialog::text("O que deseja fazer ?")
            .title("Rzip")
            .button("Listar",listar)
            //.button("Extrair",extrair)
            .button("Sair",|i| i.quit()));
    }

    pub fn listar(i: &mut Cursive) {
        i.pop_layer();
        let mut select = SelectView::<String>::new()
            .h_align(HAlign::Center)
            .autojump();
    
        let arq = "items_zip_rzip.txt";
        let conteudo = fs::read_to_string(arq).unwrap();
        select.add_all_str(conteudo.lines());
    
        i.add_layer(Dialog::around(select.scrollable().fixed_size((20, 10))).title("Where are you from?")
                    .title("Rzip - Listar")
                    .button("Voltar",menu_inicial));
    
    }
    
    


















}
pub mod interface {

    use cursive::views::Dialog;
    use cursive::views::TextView;
    use cursive::Cursive;
    use cursive::traits::*;
    use cursive::views::SelectView;
    use cursive::align::HAlign;

    use crate::rzip;


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
            .button("Extrair",extrair)
            .button("Sair",|i| i.quit()));
    }

    pub fn listar(i: &mut Cursive) {
        use std::{fs::File, path::Path,env};

        let opc: Vec<String> = env::args().collect();
        let nome_arquivo = &opc[1];
        let path_arq = Path::new(&nome_arquivo);
        let arq = File::open(&path_arq).expect("Arquivo não encontrado");

        i.pop_layer();
        let mut select = SelectView::<String>::new()
            .h_align(HAlign::Center)
            .autojump();
    
        let arq: Vec<String> = rzip::rzip::listar_arquivos_interface(&arq);
        let conteudo = arq;

        for item in &conteudo {
            select.add_item(item, item.clone());
        }
    
        i.add_layer(Dialog::around(select.scrollable().fixed_size((20, 10))).title("Where are you from?")
                    .title("Rzip - Listar arquivos")
                    .button("Voltar",menu_inicial));
    
    }

    pub fn extrair(i: &mut Cursive) {
        use std::{fs::File, path::Path,env};

        let opc: Vec<String> = env::args().collect();
        let nome_arquivo = &opc[1];
        let path_arq = Path::new(&nome_arquivo);
        let arq = File::open(&path_arq).expect("Arquivo não encontrado");
        let len = rzip::rzip::numero_de_arquivos(&arq);

        i.pop_layer();

        i.add_layer(Dialog::around(TextView::new(format!("Tem certeza que deseja extrair?\nSerão extraídos {len} arquivos")))
                    .title("Rzip - Extrair arquivos")
                    .button("Sim", extrair_arq)
                    .button("Não", menu_inicial));

        fn extrair_arq(i: &mut Cursive) {
            let opc: Vec<String> = env::args().collect();
            let nome_arquivo = &opc[1];
            let path_arq = Path::new(&nome_arquivo);
            let arq = File::open(&path_arq).expect("Arquivo não encontrado");
            rzip::rzip::extrair_arquivos_interface(&arq);

            i.pop_layer();

            i.add_layer(Dialog::around(TextView::new("Tarefa concluída com sucessso"))
                        .title("Rzip - Extrair arquivos")
                        .button("Voltar", menu_inicial)
                        .button("Sair", |i| i.quit()));
        }
    }
}
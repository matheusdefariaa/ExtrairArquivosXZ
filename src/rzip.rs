pub mod rzip {
    // Função para extrair arquivos
    pub fn extrair_arquivos() -> i32 {
        0
    }

    // Função para listar arquivos dentro do arquivo zip
    pub fn listar_arquivos(nome: &std::fs::File) -> i32 {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        println!("{}","-".repeat(17));
        println!("Lista de arquivos");
        println!("{}","-".repeat(17));
        
        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let mut nome_arqs = arq_zip.by_index(x).unwrap();
            
            // Exibir os nomes dos arquivos
            match nome_arqs.enclosed_name() {
                Some(nome) => println!("{}",nome.display()),
                None => continue,
            }
        }
        0
    }
}
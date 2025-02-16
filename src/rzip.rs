pub mod rzip {

    // Função para extrair arquivos
    pub fn extrair_arquivos(nome: &std::fs::File) -> i32 {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        for x in 0..arq_zip.len() {
            let nome_arqs = arq_zip.by_index(x).unwrap();

            let caminho_arq = match nome_arqs.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if (*nome_arqs.name()).ends_with("/") {
                println!("Arquivos extraidos para {}",caminho_arq.display());
                std::fs::create_dir_all(&caminho_arq).unwrap();
            }
        }
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
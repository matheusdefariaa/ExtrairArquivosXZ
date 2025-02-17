pub mod rzip {

    // Função para extrair arquivos
    pub fn extrair_arquivos(nome: &std::fs::File) -> i32 {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let mut nome_arqs = arq_zip.by_index(x).unwrap();

            // Exibir o caminho dos arquivos
            let caminho_arq = match nome_arqs.enclosed_name() {
                Some(caminho) => caminho.to_owned(),
                None => continue,
            };

            // Verifica e cria diretórios
            if (*nome_arqs.name()).ends_with("/") {
                println!("Arquivos extraidos para {}",caminho_arq.display());
                std::fs::create_dir_all(&caminho_arq).unwrap();
            }

            else {
                // Verifica e cria diretórios
                if let Some(c) = caminho_arq.parent() {
                    if !c.exists() {
                        std::fs::create_dir_all(&c).unwrap();
                    }
                }

                // Arquivo de saida
                let mut arq_saida = std::fs::File::create(&caminho_arq).unwrap();

                // Copia o conteúdo de nome_arqs para arq_saida
                std::io::copy(&mut nome_arqs, &mut arq_saida).unwrap();
            }
        }
        0
    }

    // Função para listar arquivos dentro do arquivo zip
    pub fn listar_arquivos(nome: &std::fs::File) -> i32 {
        use std::fs::{self, OpenOptions};
        use std::io::Write;

        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        println!("{}","-".repeat(17));
        println!("Lista de arquivos");
        println!("{}","-".repeat(17));

        let mut arquivo_salvo = fs::OpenOptions::new()
        .append(true)
        .create(true) // Cria o arquivo se ele não existir
        .open("items_zip_rzip.txt")
        .expect("Erro ao abrir o arquivo para adição");
        
        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let nome_arqs = arq_zip.by_index(x).unwrap();
            
            // Exibir os nomes dos arquivos
            match nome_arqs.enclosed_name() {
                Some(nome) => {
                    println!("{}",nome.display());
                    writeln!(arquivo_salvo, "{}", nome.display().to_string()).expect("Erro ao escrever no arquivo");

                },
                None => continue,
            }
        }
        0
    }
}
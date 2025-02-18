pub mod rzip {

    // Função para extrair arquivos
    pub fn extrair_arquivos(nome: &std::fs::File) -> i32 {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");
        let mut len = numero_de_arquivos(nome);
        let mut opc = String::new();

        println!("\n\t\t📂 Extrair arquivos 📂\n");
        
        println!("\nNúmero de arquivos que serão extraídos: {len}");
        println!("Deseja continuar(S/N)?\n");
        std::io::stdin()
            .read_line(&mut opc)
            .expect("Erro ao ler entrada");

        let opc = opc.trim().to_lowercase();

        if opc != "s" {
            println!("Operação cancelada com sucesso");
            return 0
        }
        

        // Percorre todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let mut nome_arqs = arq_zip.by_index(x).unwrap();

            // Exibir o caminho dos arquivos
            let caminho_arq = match nome_arqs.enclosed_name() {
                Some(caminho) => caminho.to_owned(),
                None => continue,
            };

            // Verifica e cria diretórios
            if (*nome_arqs.name()).ends_with("/") {
                println!("📁 Diretório extraído: ↪️ {}",caminho_arq.display());
                std::fs::create_dir_all(&caminho_arq).unwrap();
            }

            else {
                // Verifica e cria diretórios
                if let Some(c) = caminho_arq.parent() {
                    if !c.exists() {
                        std::fs::create_dir_all(&c).unwrap();
                        if (*nome_arqs.name()).contains("/") {

                            println!("📁 / 📃 Pasta e arquivo extraídos: ↪️ {}",caminho_arq.display());
                        }
                        else {
                            println!("📃 Arquivo extraído: ↪️ {}",caminho_arq.display());
                        }  
                    }
                }


            // Arquivo de saida
            let mut arq_saida = std::fs::File::create(&caminho_arq).unwrap();

            // Copia o conteúdo de nome_arqs para arq_saida
            std::io::copy(&mut nome_arqs, &mut arq_saida).unwrap();
            }
        }
        println!("\nNúmero de arquivos extraídos: {len}");
        0
    }

    // Função para extrair arquivos para interface
    pub fn extrair_arquivos_interface(nome: &std::fs::File) {
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
    }

    // Função para listar arquivos dentro do arquivo zip
    pub fn listar_arquivos(nome: &std::fs::File) -> i32 {

        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        println!("\n\t\t📋 Lista de arquivos 📋\n");
        
        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let nome_arqs = arq_zip.by_index(x).unwrap();
            
            // Exibir os nomes dos arquivos
            match nome_arqs.enclosed_name() {
                Some(nome) => {
                    if (*nome_arqs.name()).contains("/") {
                        println!("📁 ↪️ {}",nome.display());
                        continue;
                    }
                    println!("📃 ↪️ {}",nome.display());
                },
                None => continue,
            }
        }
        0
    }

    // Função para listar arquivos dentro do arquivo zip para interface
    pub fn listar_arquivos_interface(nome: &std::fs::File) -> Vec<String> {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");
        let mut nomes_arquivos: Vec<String> = Vec::new();

        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let nome_arqs = arq_zip.by_index(x).unwrap();
            
            // Exibir os nomes dos arquivos
            match nome_arqs.enclosed_name() {
                Some(nome) => nomes_arquivos.push(nome.to_string_lossy().to_string()),
                None => continue,
            }
        }
        nomes_arquivos
    }

    pub fn numero_de_arquivos(nome: &std::fs::File) -> usize {
        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");
        let mut len: usize = 0;

        for x in 0..arq_zip.len() {
            len += 1
        }

        len
    }
}
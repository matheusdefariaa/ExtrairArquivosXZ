pub mod rzip {

    // Função para extrair arquivos
    pub fn extrair_arquivos(nome: &std::fs::File) -> i32 {
        use colored::Colorize;

        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");
        let len = numero_de_arquivos(nome);
        let mut opc = String::new();

        println!("{}\n","Extrair arquivos 📂".white().bold());
        
        println!("{} {} {}","Tem certeza que deseja extrair?\nSerão extraídos".green().bold(),len.to_string().green().bold(),"arquivos".green().bold());
        println!("{}","Deseja continuar(S/N)?".white().bold());
        std::io::stdin()
            .read_line(&mut opc)
            .expect("Erro ao ler entrada");

        let opc = opc.trim().to_lowercase();

        if opc != "s" {
            println!("{}","Operação cancelada com sucesso".red().bold());
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

            let c_string = &caminho_arq.to_string_lossy();
            // Verifica e cria diretórios
            if (*nome_arqs.name()).ends_with("/") {
                println!("{} {} {}","📁".green(),"↪".green(),c_string.blue().bold().italic());
                std::fs::create_dir_all(&caminho_arq).unwrap();
            }

            else {
                // Verifica e cria diretórios
                if let Some(c) = caminho_arq.parent() {
                    if !c.exists() {
                        std::fs::create_dir_all(&c).unwrap();
                        if (*nome_arqs.name()).contains("/") {
                            println!("{} {}","📁 / 📃 Pasta e arquivo extraídos: ↪".green(),c_string.blue().bold().italic());
                        }
                        else {
                            println!("{} {} {} {}","📃".green(),"Arquivo extraído:".green(),"↪".green(),c_string.white());
                        }  
                    }
                }

            // Arquivo de saida
            let mut arq_saida = std::fs::File::create(&caminho_arq).unwrap();

            // Copia o conteúdo de nome_arqs para arq_saida
            std::io::copy(&mut nome_arqs, &mut arq_saida).unwrap();
            }
        }
        println!("\n{} {}","Número de arquivos extraídos:".white().bold(),len.to_string().blue().bold());
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
        use colored::Colorize;

        let mut arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        println!("{}\n","Lista de arquivos 📋".white().bold());
        
        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            let nome_arqs = arq_zip.by_index(x).unwrap();
            
            // Exibir os nomes dos arquivos
            match nome_arqs.enclosed_name() {
                Some(nome) => {
                    let n_string = &nome.to_string_lossy();
                    if (*nome_arqs.name()).contains("/") {
                        println!("{} {} {}","📁".green(),"↪".green(),n_string.blue().bold().italic());
                        continue;
                    }
                    println!("{} {} {}","📃".green(),"↪".green(),n_string.white());
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

        // Função para pegar o número de arquivos dentro do arquivo zip
    pub fn numero_de_arquivos(nome: &std::fs::File) -> usize {
        let arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");
        let mut len: usize = 0;

        for _x in 0..arq_zip.len() {
            len += 1
        }

        len
    }
}
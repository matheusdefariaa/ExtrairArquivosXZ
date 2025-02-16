pub mod rzip {
    // Função para extrair arquivos
    pub fn extrair_arquivos() -> i32 {
        0
    }

    // Função para listar arquivos dentro do arquivo zip
    pub fn listar_arquivos(nome: &std::fs::File) -> i32 {
        let arq_zip = zip::ZipArchive::new(nome).expect("Erro ao ler zip");

        println!("{}","-".repeat(17));
        println!("Lista de arquivos");
        println!("{}","-".repeat(17));
        
        // Imprimi na tela todos os arquivos dentro do arquivo zip
        for x in 0..arq_zip.len() {
            println!("{x}");
        }
        0
    }
}
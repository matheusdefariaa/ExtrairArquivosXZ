use std::{fs, io, path::Path,env};

fn main() {
    // Chama a função para extair arquivos 
    std::process::exit(extrair_arquivos());
}


// Função para extrair arquivos
fn extrair_arquivos() -> i32 {
    let nome_arquivo = "arq.zip";
    let path_arq = Path::new(&nome_arquivo);

    let mut arq = fs::File::open(&path_arq).unwrap();
    
    let mut arq_zip = zip::ZipArchive::new(arq).expect("Erro ao ler zip");

    for x in 0..arq_zip.len() {
        println!("{x}");
    }
    0
}

// Função para extrair arquivos dentro do arquivo zip
fn listar_arquivos() {
    todo!()
}

// Função de ajuda
fn help() {
    todo!()
}
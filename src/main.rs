use std::{env, fs::File, path::Path, process};

fn main() {
    // Pegar a entrada do usuário
    let opc: Vec<String> = env::args().collect();

    // Verifica se encontrou a flag e o nome do arquivo zip
    if opc.len() != 3 {
        process::exit(help());
    }

    // Verificar o nome do arquivo zip
    let nome_arquivo = &opc[2];

    let path_arq = Path::new(&nome_arquivo);

    // Abir o arquivo zip
    let arq = File::open(&path_arq).expect("Arquivo não encontrado");

    // Verificar a flag
    let opc = &opc[1];

    // Chama a função para listar os arquivos 
    if opc == "-l" {
        process::exit(listar_arquivos(&arq));
    }

    // Chama a função para extair os arquivos 
    else if opc == "-d" {
        extrair_arquivos();
    }

    else if opc == "-h" {
        process::exit(help());
    }

    else {
        println!("{}","-".repeat(64));
        println!("\tDigite a flag [-h] para ajuda");
        println!("{}","-".repeat(64));
    }
}


// Função para extrair arquivos
fn extrair_arquivos() -> i32 {
    0
}

// Função para listar arquivos dentro do arquivo zip
fn listar_arquivos(nome: &File) -> i32 {
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

// Função de ajuda
fn help() -> i32 {
    println!("{}","-".repeat(64));
    println!("\tAutor: Matheus de Faria");
    println!("\tVersão: 1.0");
    println!("\tData de lançamento: 15/02/2025\n");
    println!("\tPrograma para listar e descomprimir arquivos zip");
    println!("\t\tHELP [-h]");
    println!("\t\tListar arquivos [-l]");
    println!("\t\tDescomprimir arquivos [-d]");
    println!("{}","-".repeat(64));
    0
}
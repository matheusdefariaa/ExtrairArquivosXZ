use std::{env, fs::File, path::Path, process};

mod rzip;
mod interface;

fn main() {
    // Pegar a entrada do usuário
    let opc: Vec<String> = env::args().collect();

    if opc.len() == 2 {
        let nome_arquivo = &opc[1];
        if nome_arquivo.contains(".zip") || nome_arquivo.contains(".7z") {
            interface::interface::interface_main();
            process::exit(0);
        }

    }

    // Verifica se encontrou a flag e o nome do arquivo zip
    if opc.len() != 3 {
        let nome_arquivo = &opc[1];
        if nome_arquivo == "-v" {
            process::exit(version());
        }

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
        process::exit(rzip::rzip::listar_arquivos(&arq));
    }

    // Chama a função para extair os arquivos 
    else if opc == "-d" {
        process::exit(rzip::rzip::extrair_arquivos(&arq));
    }

    else if opc == "-h" {
        process::exit(help());
    }

    else if opc == "-v" {
        process::exit(version());
    }

    else {
        println!("{}","-".repeat(64));
        println!("\tDigite a flag [-h] para ajuda");
        println!("{}","-".repeat(64));
    }
}

// Função de ajuda
fn help() -> i32 {
    println!("Uso: rzip [OPÇÃO]... [ARQUIVO]...");
    println!("Lista e extrai arquivos zip.");
    println!("Abre a interface gráfica se não for usada nenhuma opção -dhlv.");
    println!("  -d {} extrair arquivos", " ".repeat(10));
    println!("  -h {} exibir esta ajuda e sai"," ".repeat(10));
    println!("  -l {} listar arquivos"," ".repeat(10));
    println!("  -v {} exibe a versão e sai", " ".repeat(10));
    println!("Abrir interface no terminal: rzip <nome do arquivo>");
    0
}

fn version() -> i32 {
    println!("Versão: 1.2");
    println!("Data de lançamento: 15/02/2025");
    println!("Escrito por Matheus de Faria.");
    println!("Atualização: 20/02/2025");
    0
}
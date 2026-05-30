use clap::Parser;
use oak_utils::loterias::{Jogo, Sorteio, formatar_reais};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    jogo: Jogo,
}

fn main() {
    let cli = Cli::parse();
    let nome = cli.jogo.nome();
    let preco = cli.jogo.preco_centavos();

    let sorteio = Sorteio::new(cli.jogo);
    let sorteadas = sorteio.sortear();
    println!("Dezenas sorteadas para o jogo {nome}:");
    println!("{sorteadas:?}");
    println!();

    if sorteio.is_lotomania_espelho() {
        let espelho = Sorteio::sortear_espelho(&sorteadas);
        println!("Dezenas sorteadas para o jogo {nome} (espelho):");
        println!("{espelho:?}");
        println!();
        println!("Preço total: {}.", formatar_reais(2 * preco));
    } else {
        println!("Preço total: {}.", formatar_reais(preco));
    }
}

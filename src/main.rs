use clap::Parser;
use oak_utils::loterias::{Jogo, Sorteio, formatar_reais};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    jogo: Option<Jogo>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(jogo) = cli.jogo {
        let nome = jogo.nome();
        let preco = jogo.preco_centavos();

        let sorteadas = Sorteio::new(jogo).sortear();
        println!("Dezenas sorteadas para o jogo {nome}:\n{sorteadas:?}");
        println!("Preço total: {}.", formatar_reais(preco));
    }
}

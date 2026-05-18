use clap::Subcommand;
use rand::prelude::*;

/// # Errors
/// Mensagem de erro para erros de conversão ou para o total de dezenas válidas fora do intervalo do jogo.
fn validar_dezenas_megasena(total_dezenas: &str) -> Result<u8, String> {
    let total = total_dezenas
        .parse::<u8>()
        .map_err(|_| "Número inválido.")?;
    if (6..=20).contains(&total) {
        Ok(total)
    } else {
        Err("Total de dezenas está fora do intervalo de 6 a 20 números.".to_string())
    }
}

/// # Errors
/// Mensagem de erro para erros de conversão ou para o total de dezenas válidas fora do intervalo do jogo.
fn validar_dezenas_quina(total_dezenas: &str) -> Result<u8, String> {
    let total = total_dezenas
        .parse::<u8>()
        .map_err(|_| "Número inválido.")?;
    if (5..=15).contains(&total) {
        Ok(total)
    } else {
        Err("Total de dezenas está fora do intervalo de 5 a 15 números".to_string())
    }
}

#[derive(Debug, Subcommand)]
pub enum Jogo {
    Lotomania,
    Megasena {
        #[arg(value_parser = validar_dezenas_megasena)]
        total_dezenas: u8,
    },
    Quina {
        #[arg(value_parser = validar_dezenas_quina)]
        total_dezenas: u8,
    },
}

impl Jogo {
    #[must_use]
    pub const fn nome(&self) -> &'static str {
        match self {
            Self::Lotomania => "lotomania",
            Self::Megasena { .. } => "megasena",
            Self::Quina { .. } => "quina",
        }
    }

    #[must_use]
    pub const fn preco_base_centavos(&self) -> u32 {
        match self {
            Self::Lotomania | Self::Quina { .. } => 300,
            Self::Megasena { .. } => 600,
        }
    }

    #[must_use]
    pub const fn configurar_dezenas(&self) -> (u8, u8) {
        match self {
            Self::Lotomania => (50, 50),
            Self::Megasena { total_dezenas } => (*total_dezenas, 6),
            Self::Quina { total_dezenas } => (*total_dezenas, 5),
        }
    }

    #[must_use]
    pub fn preco_centavos(&self) -> u64 {
        let (total, minimo) = self.configurar_dezenas();
        let apostas = combinacoes(u64::from(total), u64::from(minimo));
        apostas * u64::from(self.preco_base_centavos())
    }
}

fn combinacoes(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    (1..=k).fold(1u64, |acc, i| acc * (n - i + 1) / i)
}

#[derive(Debug)]
pub struct Sorteio {
    jogo: Jogo,
}

impl Sorteio {
    #[must_use]
    pub const fn new(jogo: Jogo) -> Self {
        Self { jogo }
    }

    #[must_use]
    pub fn sortear(&self) -> Vec<u8> {
        match self.jogo {
            Jogo::Lotomania => Self::dezenas(50, 100),
            Jogo::Megasena { total_dezenas } => Self::dezenas(total_dezenas, 60),
            Jogo::Quina { total_dezenas } => Self::dezenas(total_dezenas, 80),
        }
    }

    #[must_use]
    pub fn dezenas(total_dezenas: u8, dezena_maxima: u8) -> Vec<u8> {
        let mut rng = rand::rng();
        let mut sorteio = (1..=dezena_maxima).sample(&mut rng, usize::from(total_dezenas));
        sorteio.sort_unstable();
        sorteio
    }
}

#[must_use]
pub fn formatar_reais(centavos: u64) -> String {
    format!("R$ {},{:02}", centavos / 100, centavos % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn megasena_simples_custa_seis_reais() {
        let j = Jogo::Megasena { total_dezenas: 6 };
        assert_eq!(j.preco_centavos(), 600);
    }

    #[test]
    fn megasena_oito_dezenas_custa_cento_e_sessenta_e_oito_reais() {
        let j = Jogo::Megasena { total_dezenas: 8 };
        assert_eq!(j.preco_centavos(), 16800); // C(8,6)=28 * 600
    }

    #[test]
    fn quina_oito_dezenas_custa_cento_e_sessenta_e_oito_reais() {
        let j = Jogo::Quina { total_dezenas: 8 };
        assert_eq!(j.preco_centavos(), 16800);
    }

    #[test]
    fn quina_minima_custa_tres_reais() {
        let j = Jogo::Quina { total_dezenas: 5 };
        assert_eq!(j.preco_centavos(), 300);
    }

    #[test]
    fn combinacoes_basicas() {
        assert_eq!(combinacoes(6, 6), 1);
        assert_eq!(combinacoes(8, 6), 28);
        assert_eq!(combinacoes(20, 6), 38760);
    }
    #[test]

    fn sorteio_quina_retorna_quantidade_correta() {
        let s = Sorteio::new(Jogo::Quina { total_dezenas: 7 });
        let r = s.sortear();
        assert_eq!(r.len(), 7);
    }

    #[test]
    fn sorteio_megasena_nao_repete_dezenas() {
        let s = Sorteio::new(Jogo::Megasena { total_dezenas: 10 });
        let r = s.sortear();
        let unicos: std::collections::HashSet<_> = r.iter().collect();
        assert_eq!(unicos.len(), r.len());
    }

    #[test]
    fn sorteio_lotomania_respeita_limite_superior() {
        let r = Sorteio::new(Jogo::Lotomania).sortear();
        assert!(r.iter().all(|&d| (1..=100).contains(&d)));
    }

    #[test]
    fn sorteio_retorna_ordenado() {
        let r = Sorteio::new(Jogo::Quina { total_dezenas: 10 }).sortear();
        assert!(r.windows(2).all(|w| w[0] < w[1]));
    }
}

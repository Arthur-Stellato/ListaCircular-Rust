use std::io;
use std::io::Write;

const MAX: usize = 5;

struct ListaCircular<T: Copy + std::fmt::Display + PartialEq> {
    nos: [Option<T>; MAX],
    prox: [usize; MAX],
    cabeca: Option<usize>,
    tamanho: usize,
}

impl<T: Copy + std::fmt::Display + PartialEq> ListaCircular<T> {
    pub fn new() -> Self {
        ListaCircular {
            nos: [None; MAX],
            prox: [0; MAX],
            cabeca: None,
            tamanho: 0,
        }
    }

    //Auxiliar para encontrar o primeiro slot livre no array de nós
    fn achar_slot_livre(&self) -> Option<usize> {
        for i in 0..MAX { 
            if self.nos[i].is_none() {
                return Some(i);
            }
        }
        None // nenhum slot livre encontrado
    }

    //Auxiliar para encontrar o índice da cauda (último elemento) da lista
    fn achar_cauda(&self) -> usize {
        let cab = self.cabeca.unwrap(); //Começa pela cabeça
        let mut idx = cab;
        while self.prox[idx] != cab { // enquanto não voltar à cabeça
            idx = self.prox[idx]; // Avança para o próximo
        }
        idx
    }

    // Auxiliar para encontrar o índice do elemento na posição lógica dada
    fn idx_na(&self, pos: usize) -> usize {
        let mut idx = self.cabeca.unwrap();  // começa na cabeça (posição 0)
        for _ in 0..pos { // avança 'pos' vezes
            idx = self.prox[idx];
        }
        idx
    }

    pub fn inserir_inicio(&mut self, valor: T) {
        if self.tamanho == MAX {
            println!(" lista cheia (máximo: {})", MAX);
            return;
        }
        let slot = self.achar_slot_livre().unwrap();
        self.nos[slot] = Some(valor); // armazena o valor no slot encontrado
        if self.tamanho == 0 {
            self.prox[slot] = slot;
            self.cabeca = Some(slot);
        } else {
            let cabeca = self.cabeca.unwrap();
            let cauda = self.achar_cauda();
            self.prox[slot] = cabeca;
            self.prox[cauda] = slot;
            self.cabeca = Some(slot);
        }
        self.tamanho += 1;
        println!("{} inserido no início", valor);
    }

    pub fn inserir_fim(&mut self, valor: T) {
        if self.tamanho == MAX {
            println!("lista cheia (máximo: {})", MAX);
            return;
        }
        let slot = self.achar_slot_livre().unwrap();
        self.nos[slot] = Some(valor);
        if self.tamanho == 0 {
            self.prox[slot] = slot;
            self.cabeca = Some(slot);
        } else {
            let cabeca = self.cabeca.unwrap();
            let cauda = self.achar_cauda();
            self.prox[slot] = cabeca;
            self.prox[cauda] = slot;
        }
        self.tamanho += 1;
        println!("{} inserido no fim", valor);
    }

    // recebe uma posição e retorna o valor da posição seguinte
    pub fn ler_proximo(&self, pos: usize) -> Option<T> {
        if self.tamanho == 0 || pos >= self.tamanho {
            return None;
        }
        let idx = self.idx_na(pos);
        // self.prox[idx] já aponta para a cabeça se for a cauda
        // o comportamento circular acontece naturalmente
        if self.prox[idx] == self.cabeca.unwrap() {
            println!("próximo de [{}] é a cabeça da lista", pos);
        }
        self.nos[self.prox[idx]]
    }

    pub fn remover(&mut self, pos: usize) -> Option<T> {
        // CENÁRIO A — posição inválida
        if pos >= self.tamanho {
            return None;
        }

        // CENÁRIO B — lista com UM único elemento
        if self.tamanho == 1 {
            let idx = self.cabeca.unwrap();
            let valor = self.nos[idx].take();
            self.cabeca = None;
            self.tamanho = 0;
            return valor;
        }
        
        // CENÁRIO C — lista com 2+ elementos
        let (rem, ant) = if pos == 0 {
            (self.cabeca.unwrap(), self.achar_cauda())
        } else {
            let a = self.idx_na(pos - 1);  // acha o antecessor
            (self.prox[a], a)  // (nó a remover, antecessor)
        };

        self.prox[ant] = self.prox[rem];  // pula o nó removido
        if pos == 0 {
            self.cabeca = Some(self.prox[rem]); // atualiza cabeça
        }
        let valor = self.nos[rem].take();
        self.tamanho -= 1;
        valor
    }

    pub fn percorrer(&self) {
        if self.tamanho == 0 {
            println!("lista vazia");
            return;
        }

        // primeira passagem: valores
        print!("  HEAD → ");
        let mut idx = self.cabeca.unwrap();
        for i in 0..self.tamanho {
            print!("[{}]", self.nos[idx].unwrap());
            if i < self.tamanho - 1 {
                print!(" → ");
            }
            idx = self.prox[idx];
        }
        println!(" → (HEAD)");

        // segunda passagem: índices físicos
        print!("        ");
        idx = self.cabeca.unwrap();
        for i in 0..self.tamanho {
            print!(" {} ", idx);
            if i < self.tamanho - 1 {
                print!("    ");
            }
            idx = self.prox[idx];
        }
        println!();

        println!("  tamanho: {}/{}", self.tamanho, MAX);
    }

    // mostra o estado físico dos dois arrays internos
    pub fn tabela_fisica(&self) {
        println!("  idx │ nos      │ prox");
        println!("  ────┼──────────┼──────");
        for i in 0..MAX {
            let val = match self.nos[i] {
                Some(v) => format!("{:>8}", v),
                None => "  vazio  ".to_string(),
            };
            let head = if self.cabeca == Some(i) {
                " ← HEAD"
            } else {
                ""
            };
            println!("   {}  │ {}  │  {}   │{}", i, val, self.prox[i], head);
        }
    }
}

// lê uma linha do terminal e já converte para o tipo pedido
fn ler(mensagem: &str) -> String {
    print!("  {}: ", mensagem);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn main() {
    let mut lista: ListaCircular<i32> = ListaCircular::new();

    loop {
        println!("╔══════════════════════════════╗");
        println!("║     LISTA CIRCULAR ESTÁTICA  ║");
        println!("╠══════════════════════════════╣");
        println!("║  1 - Inserir no início       ║");
        println!("║  2 - Inserir no fim          ║");
        println!("║  3 - Ler próximo de uma pos  ║");
        println!("║  4 - Remover por posição     ║");
        println!("║  5 - Mostrar lista           ║");
        println!("║  6 - Tabela física           ║");
        println!("║  0 - Sair                    ║");
        println!("╚══════════════════════════════╝");
        print!("  Escolha: ");
        io::stdout().flush().unwrap();

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        println!();

        match opcao {
            "1" => {
                let v: i32 = ler("valor").parse().unwrap();
                lista.inserir_inicio(v);
            }
            "2" => {
                let v: i32 = ler("valor").parse().unwrap();
                lista.inserir_fim(v);
            }
            "3" => {
                let pos: usize = ler("posição").parse().unwrap();
                match lista.ler_proximo(pos) {
                    Some(v) => println!("próximo de [{}] = {}", pos, v),
                    None => println!("posição {} não existe (tamanho: {})", pos, lista.tamanho),
                }
            }
            "4" => {
                let pos: usize = ler("posição").parse().unwrap();
                match lista.remover(pos) {
                    Some(v) => println!("removido: {}", v),
                    None => println!("posição {} não existe (tamanho: {})", pos, lista.tamanho),
                }
            }
            "5" => {
                lista.percorrer();
            }
            "6" => {
                lista.tabela_fisica();
            }
            "0" => {
                println!("  Saindo...");
                break;
            }
            _ => {
                println!("opção inválida, tente novamente");
            }
        }
    }
}
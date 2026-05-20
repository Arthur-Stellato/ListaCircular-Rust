use std::io;
use std::io::Write; // necessário para flush do print!

// ── tamanho máximo da lista ──────────────────────────────────
const MAX: usize = 5;

// ── Estrutura ────────────────────────────────────────────────
struct ListaCircular<T: Copy + std::fmt::Display + PartialEq> {
    nos:     [Option<T>; MAX],
    prox:    [usize; MAX],
    cabeca:  Option<usize>,
    tamanho: usize,
}

impl<T: Copy + std::fmt::Display + PartialEq> ListaCircular<T> {

    pub fn new() -> Self {
        ListaCircular {
            nos:     [None; MAX],
            prox:    [0; MAX],
            cabeca:  None,
            tamanho: 0,
        }
    }

    fn achar_slot_livre(&self) -> Option<usize> {
        for i in 0..MAX {
            if self.nos[i].is_none() { return Some(i); }
        }
        None
    }

    fn achar_cauda(&self) -> usize {
        let cab = self.cabeca.unwrap();
        let mut idx = cab;
        while self.prox[idx] != cab { idx = self.prox[idx]; }
        idx
    }

    fn idx_na(&self, pos: usize) -> usize {
        let mut idx = self.cabeca.unwrap();
        for _ in 0..pos { idx = self.prox[idx]; }
        idx
    }

    pub fn inserir_inicio(&mut self, valor: T) {
        if self.tamanho == MAX { // lista cheia?
            println!("  ✗ lista cheia (máximo: {})", MAX);
            return;
        }

        let slot = self.achar_slot_livre().unwrap(); // acha espaço vazio
        self.nos[slot] = Some(valor); // guarda o valor

        if self.tamanho == 0 {
            self.prox[slot] = slot;  // aponta para si mesmo (Head)
            self.cabeca = Some(slot);
        } else {
            let cabeca = self.cabeca.unwrap();
            let cauda  = self.achar_cauda();
            self.prox[slot] = cabeca; // novo → antiga cabeça
            self.prox[cauda] = slot; // cauda → novo
            self.cabeca = Some(slot);  // novo vira a cabeça
        }

        self.tamanho += 1;
        println!("  ✓ {} inserido no início", valor);
    }

    pub fn inserir_fim(&mut self, valor: T) { // Mesma coisa que inserir_inicio, mas o novo nó vira a cauda
        if self.tamanho == MAX {
            println!("  ✗ lista cheia (máximo: {})", MAX);
            return;
        }
        let slot = self.achar_slot_livre().unwrap();
        self.nos[slot] = Some(valor);
        if self.tamanho == 0 {
            self.prox[slot] = slot;
            self.cabeca = Some(slot);
        } else {
            let cabeca = self.cabeca.unwrap();
            let cauda  = self.achar_cauda();
            self.prox[slot] = cabeca;
            self.prox[cauda] = slot;
        }
        self.tamanho += 1;
        println!("  ✓ {} inserido no fim", valor);
    }

    pub fn ler(&self, pos: usize) -> Option<T> {
        if pos >= self.tamanho { return None; }
        self.nos[self.idx_na(pos)]
    }

    pub fn remover(&mut self, pos: usize) -> Option<T> {
        if pos >= self.tamanho { return None; } // posição inválida?

        if self.tamanho == 1 { // único nó?
            let idx = self.cabeca.unwrap();
            let valor = self.nos[idx].take(); // take() retira o valor e deixa None
            self.cabeca = None;
            self.tamanho = 0;
            return valor;
        }

        let (rem, ant) = if pos == 0 { // removendo a cabeça?
            (self.cabeca.unwrap(), self.achar_cauda())
        } else {
            let a = self.idx_na(pos - 1); // acha o anterior
            (self.prox[a], a)
        };

        self.prox[ant] = self.prox[rem]; // anterior pula o removido
        if pos == 0 { self.cabeca = Some(self.prox[rem]); } // atualiza cabeça
        let valor = self.nos[rem].take(); // libera o slot
        self.tamanho -= 1;
        valor
    }

    pub fn percorrer(&self) {
        if self.tamanho == 0 {
            println!("  lista vazia");
            return;
        }
        print!("  HEAD → ");
        let mut idx = self.cabeca.unwrap();
        for i in 0..self.tamanho { // exatamente N passos
            print!("[{}]", self.nos[idx].unwrap());
            if i < self.tamanho - 1 { print!(" → "); }
            idx = self.prox[idx]; // avança para o próximo
        }
        println!(" → (HEAD)");
        println!("  tamanho: {}/{}", self.tamanho, MAX);
    }
}

// ── Funções do menu ──────────────────────────────────────────

// imprime o menu de opções
fn imprimir_menu() {
    println!("\n╔══════════════════════════════╗");
    println!("║     LISTA CIRCULAR ESTÁTICA  ║");
    println!("╠══════════════════════════════╣");
    println!("║  1 - Inserir no início       ║");
    println!("║  2 - Inserir no fim          ║");
    println!("║  3 - Ler por posição         ║");
    println!("║  4 - Remover por posição     ║");
    println!("║  5 - Mostrar lista           ║");
    println!("║  0 - Sair                    ║");
    println!("╚══════════════════════════════╝");
    print!("  Escolha: ");
    // flush garante que o print! apareça antes de ler a entrada
    io::stdout().flush().unwrap();
}

// lê uma linha do terminal e converte para i32
// retorna None se a entrada não for um número válido
fn ler_i32(mensagem: &str) -> Option<i32> {
    print!("  {}: ", mensagem);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap(); // lê o que o usuário digitou

    // trim() remove o \n do final da linha e parse::<i32>() tenta converter para i32
    entrada.trim().parse::<i32>().ok()
}

// lê uma linha do terminal e converte para usize (índice)
fn ler_usize(mensagem: &str) -> Option<usize> {
    print!("  {}: ", mensagem);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    entrada.trim().parse::<usize>().ok()
}

// ── Main com loop de menu ────────────────────────────────────
fn main() {
    let mut lista: ListaCircular<i32> = ListaCircular::new();

    // loop equivalente ao do-while: executa pelo menos uma vez
    // e continua enquanto o usuário não escolher 0
    loop {
        imprimir_menu();

        // lê a opção digitada
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        println!(); // linha em branco para separar

        match opcao {
            "1" => {
                // inserir no início
                match ler_i32("valor") {
                    Some(v) => lista.inserir_inicio(v),
                    None    => println!("  ✗ valor inválido"),
                }
            }
            "2" => {
                // inserir no fim
                match ler_i32("valor") {
                    Some(v) => lista.inserir_fim(v),
                    None    => println!("  ✗ valor inválido"),
                }
            }
            "3" => {
                // ler por posição
                match ler_usize("posição") {
                    Some(pos) => match lista.ler(pos) {
                        Some(v) => println!("  ler({}) = {}", pos, v),
                        None    => println!("  ✗ posição {} não existe (tamanho: {})", pos, lista.tamanho),
                    },
                    None => println!("  ✗ posição inválida"),
                }
            }
            "4" => {
                // remover por posição
                match ler_usize("posição") {
                    Some(pos) => match lista.remover(pos) {
                        Some(v) => println!("  ✓ removido: {}", v),
                        None    => println!("  ✗ posição {} não existe (tamanho: {})", pos, lista.tamanho),
                    },
                    None => println!("  ✗ posição inválida"),
                }
            }
            "5" => {
                // mostrar lista
                lista.percorrer();
            }
            "0" => {
                // sair — quebra o loop
                println!("  Saindo...");
                break;
            }
            _ => {
                // qualquer outra entrada
                println!("  ✗ opção inválida, tente novamente");
            }
        }
    }
}
# 🦀 Rust — Lista Circular Estática

Implementação de uma **Lista Circular Estática** em Rust com menu interativo no terminal.  
Desenvolvido como material de apresentação acadêmica sobre estruturas de dados.

---

## Sumário

1. [Configuração do Ambiente](#-configuração-do-ambiente)
2. [Sintaxe Básica — Variáveis](#-sintaxe-básica--variáveis)
3. [Hello World](#-hello-world)
4. [O que é uma Lista Circular](#-o-que-é-uma-lista-circular)
5. [Explicação do Código](#-explicação-do-código)
6. [Como Executar](#-como-executar)

---

## 🛠 Configuração do Ambiente

### Linux (Ubuntu, Debian, Fedora, Arch, entre outros.)

**1. Instalar o Rust com rustup**

Abra o terminal e cole o comando abaixo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Quando perguntar, pressione `1` para instalação padrão.

**2. Recarregar o PATH**

```bash
source $HOME/.cargo/env
```

**3. Verificar a instalação**

```bash
rustc --version
cargo --version
```

Saída esperada:
```
rustc 1.78.0 (9b00956e5 2024-04-29)
cargo 1.78.0 (54d8815d0 2024-04-24)
```

---

### Windows

**1. Instalar o Visual C++ Build Tools**

O Rust precisa do compilador C++ da Microsoft.  
Baixe em: https://visualstudio.microsoft.com/visual-cpp-build-tools/  
Na instalação, marque **Desktop development with C++**.

**2. Instalar o Rust**

Baixe o instalador em: https://rustup.rs  
Execute o `.exe` e pressione `1` para instalação padrão.

**3. Verificar no PowerShell**

Abra um **novo** PowerShell (para o PATH carregar) e digite:

```powershell
rustc --version
cargo --version
```

> 💡 **Dica:** instale a extensão **rust-analyzer** no VS Code para ter autocomplete, erros sublinhados e outras facilidades.

---

## 📝 Sintaxe Básica — Variáveis

Em Rust, as variáveis são **imutáveis por padrão**. Para permitir alteração, use a palavra-chave `mut`.

```rust
let x = 5;        // imutável — não pode ser alterado
let mut y = 10;   // mutável — pode ser alterado
y += 1;           // permitido
```

### Tipos comuns

```rust
let inteiro: i32   = 42;       // número inteiro com sinal
let positivo: usize = 3;       // número positivo — usado para índices
let decimal: f64   = 3.14;     // número decimal
let texto: &str    = "Rust";   // texto fixo
let booleano: bool = true;     // verdadeiro ou falso
```

### Option — ausência segura

Rust não tem `null`. Quando um valor pode ou não existir, usa-se `Option<T>`:

```rust
let com_valor: Option<i32> = Some(42);  // existe um valor
let sem_valor: Option<i32> = None;       // não existe nada
```

Para usar o valor dentro de um `Option`:

```rust
match meu_option {
    Some(v) => println!("valor: {}", v),
    None    => println!("não tem valor"),
}
```

### self

Dentro dos métodos de uma struct, `self` representa a própria instância — equivalente ao `this` do Java:

```rust
self.tamanho    // acessa o campo tamanho da struct atual
self.inserir()  // chama outro método da mesma struct
```

---

## 👋 Hello World

**1. Criar um novo projeto**

```bash
cargo new meu_projeto
cd meu_projeto
```

O Cargo gera automaticamente a estrutura:

```
meu_projeto/
├── Cargo.toml    ← metadados e dependências
└── src/
    └── main.rs   ← seu código
```

**2. Código em `src/main.rs`**

```rust
fn main() {
    // println! é uma macro — note o !
    println!("Olá, Mundo!");

    // interpolação com {}
    let nome = "Rust";
    println!("Bem-vindo ao {}!", nome);

    // interpolação direta com {variavel}
    let versao = 2024;
    println!("Versão: {versao}");
}
```

**3. Compilar e executar**

```bash
cargo run
```

Saída:
```
Olá, Mundo!
Bem-vindo ao Rust!
Versão: 2024
```

---

## 🔄 O que é uma Lista Circular

Uma lista circular é uma estrutura de dados onde **o último nó aponta de volta para o primeiro**, formando um ciclo sem fim.

```
Lista comum:    [10] → [20] → [30] → None
Lista circular: [10] → [20] → [30] → (volta para 10)
```

A diferença está em apenas um campo — o `proximo` do último nó:

```rust
// lista comum: último nó aponta para nada
No { valor: 30, proximo: None }

// lista circular: último nó aponta para a cabeça
No { valor: 30, proximo: 0 }  // índice 0 = cabeça
```

### Por que usar lista circular?

| Situação | Por quê a circular resolve |
|---|---|
| Escalonador de processos do SO | cada processo recebe um tempo de CPU e o ciclo volta automaticamente |
| Buffer de áudio/vídeo em loop | o conteúdo se repete sem lógica extra |
| Sistema de turnos em jogos | após o último jogador, volta para o primeiro |

### Lista estática vs dinâmica

| | Estática (array) | Dinâmica (Vec) |
|---|---|---|
| Tamanho | fixo — definido em `const MAX` | cresce conforme necessário |
| Memória | reservada em tempo de compilação | alocada em tempo de execução |
| Lista cheia | precisa verificar | impossível de estourar |

---

## 💻 Explicação do Código

### Estrutura dos dados

```rust
const MAX: usize = 5; // tamanho máximo da lista

struct ListaCircular<T: Copy + std::fmt::Display + PartialEq> {
    nos:     [Option<T>; MAX], // valores: Some = ocupado, None = vazio
    prox:    [usize; MAX],     // índice do próximo nó de cada posição
    cabeca:  Option<usize>,    // índice do primeiro nó
    tamanho: usize,            // quantos nós estão ocupados
}
```

Em vez de ponteiros, a lista usa **dois arrays paralelos**:

```
posição:   0         1         2         3         4
nos:   [Some(10)] [Some(20)] [Some(30)] [None]    [None]
prox:  [   1    ] [   2    ] [   0    ] [  0  ]   [  0  ]
                               ↑
                          aponta para 0 — fecha o ciclo
```

### Construtor `new`

```rust
pub fn new() -> Self {
    ListaCircular {
        nos:     [None; MAX],  // todos os slots vazios
        prox:    [0; MAX],     // índices zerados
        cabeca:  None,         // sem primeiro nó
        tamanho: 0,            // lista vazia
    }
}
```

`Self` é um atalho para o nome da própria struct. `[None; MAX]` é a sintaxe do Rust para "crie um array de MAX posições, todas com o valor None".

### Auxiliares internos

**`achar_slot_livre`** — procura uma posição `None` no array para guardar um novo nó:

```rust
fn achar_slot_livre(&self) -> Option<usize> {
    for i in 0..MAX {
        if self.nos[i].is_none() { return Some(i); }
    }
    None // todos ocupados
}
```

**`achar_cauda`** — percorre a lista até achar o nó cujo próximo é a cabeça:

```rust
fn achar_cauda(&self) -> usize {
    let cab = self.cabeca.unwrap();
    let mut idx = cab;
    while self.prox[idx] != cab { idx = self.prox[idx]; }
    idx // este nó aponta para a cabeça — é a cauda
}
```

**`idx_na`** — caminha `pos` passos a partir da cabeça e retorna o índice:

```rust
fn idx_na(&self, pos: usize) -> usize {
    let mut idx = self.cabeca.unwrap();
    for _ in 0..pos { idx = self.prox[idx]; }
    idx
}
```

### Inserir no início

```rust
pub fn inserir_inicio(&mut self, valor: T) {
    if self.tamanho == MAX { return; }           // lista cheia?

    let slot = self.achar_slot_livre().unwrap(); // acha espaço vazio
    self.nos[slot] = Some(valor);                // guarda o valor

    if self.tamanho == 0 {
        self.prox[slot] = slot;                  // único nó aponta para si
        self.cabeca = Some(slot);
    } else {
        let cabeca = self.cabeca.unwrap();
        let cauda  = self.achar_cauda();
        self.prox[slot] = cabeca;                // novo → antiga cabeça
        self.prox[cauda] = slot;                 // cauda → novo
        self.cabeca = Some(slot);                // novo vira a cabeça
    }

    self.tamanho += 1;
}
```

### Inserir no fim

Idêntico ao `inserir_inicio` com **uma única diferença** — `self.cabeca` não é atualizada, porque o início da lista continua o mesmo:

```rust
// inserir_inicio — última linha do else:
self.cabeca = Some(slot);  // ← atualiza a cabeça

// inserir_fim — não tem essa linha
//               a cabeça continua apontando para o mesmo nó
```

### Ler por posição

```rust
pub fn ler(&self, pos: usize) -> Option<T> {
    if pos >= self.tamanho { return None; } // posição inválida?
    self.nos[self.idx_na(pos)]              // caminha até a posição e retorna
}
```

Não tem acesso direto como num array — precisa caminhar nó a nó. Por isso é **O(n)**.

### Remover

```rust
pub fn remover(&mut self, pos: usize) -> Option<T> {
    if pos >= self.tamanho { return None; }  // posição inválida

    if self.tamanho == 1 {                   // único nó — limpa tudo
        let idx = self.cabeca.unwrap();
        let valor = self.nos[idx].take();    // take() tira o valor e deixa None
        self.cabeca = None;
        self.tamanho = 0;
        return valor;
    }

    // acha o nó a remover e o anterior
    let (rem, ant) = if pos == 0 {
        (self.cabeca.unwrap(), self.achar_cauda()) // remover a cabeça
    } else {
        let a = self.idx_na(pos - 1);
        (self.prox[a], a)                          // remover do meio/fim
    };

    self.prox[ant] = self.prox[rem];         // anterior pula o removido
    if pos == 0 { self.cabeca = Some(self.prox[rem]); }

    let valor = self.nos[rem].take();        // libera o slot
    self.tamanho -= 1;
    valor
}
```

O `take()` retira o valor do `Option` e deixa `None` no lugar — libera o slot para uso futuro sem precisar mover nada no array.

### Percorrer

```rust
pub fn percorrer(&self) {
    if self.tamanho == 0 { println!("lista vazia"); return; }

    let mut idx = self.cabeca.unwrap();
    for i in 0..self.tamanho {           // exatamente N passos — sem loop infinito
        print!("[{}]", self.nos[idx].unwrap());
        if i < self.tamanho - 1 { print!(" → "); }
        idx = self.prox[idx];            // avança para o próximo
    }
    println!(" → (HEAD)");
}
```

O `for` limitado ao `tamanho` é o que impede o loop infinito — sem ele, os ponteiros seriam seguidos para sempre porque o ciclo nunca termina.

### Menu interativo

O programa usa um `loop` — equivalente ao `do-while` de outras linguagens:

```rust
loop {
    imprimir_menu();                       // mostra as opções
    io::stdin().read_line(&mut opcao);     // espera o usuário digitar
    let opcao = opcao.trim();              // remove o \n do Enter

    match opcao {
        "1" => lista.inserir_inicio(v),
        "2" => lista.inserir_fim(v),
        "3" => lista.ler(pos),
        "4" => lista.remover(pos),
        "5" => lista.percorrer(),
        "0" => break,                      // sai do loop
        _   => println!("opção inválida"), // qualquer outra entrada
    }
}
```

---

## ▶ Como Executar

**Compilar e rodar com Cargo:**

```bash
cargo run
```

**Ou compilar diretamente com rustc:**

```bash
rustc lista_circular_menu.rs -o lista_circular
./lista_circular        # Linux
lista_circular.exe      # Windows
```

---

## 📚 Referências

- [Documentação oficial do Rust](https://doc.rust-lang.org)
- [The Rust Book — livro gratuito oficial](https://doc.rust-lang.org/book/)
- [rustup.rs — instalador oficial](https://rustup.rs)

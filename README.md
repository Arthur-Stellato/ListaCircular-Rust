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

### Linux (Ubuntu, Debian, Fedora, Arch, entre outros)

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
let inteiro: i32    = 42;     // número inteiro com sinal
let positivo: usize = 3;      // número positivo — usado para índices
let decimal: f64    = 3.14;   // número decimal
let texto: &str     = "Rust"; // texto fixo
let booleano: bool  = true;   // verdadeiro ou falso
```

### Option — ausência segura

Rust não tem `null`. Quando um valor pode ou não existir, usa-se `Option<T>`:

```rust
let com_valor: Option<i32> = Some(42); // existe um valor
let sem_valor: Option<i32> = None;     // não existe nada
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

| Situação                       | Por quê a circular resolve                                           |
| ------------------------------ | -------------------------------------------------------------------- |
| Escalonador de processos do SO | cada processo recebe um tempo de CPU e o ciclo volta automaticamente |
| Buffer de áudio/vídeo em loop  | o conteúdo se repete sem lógica extra                                |
| Sistema de turnos em jogos     | após o último jogador, volta para o primeiro                         |

### Lista estática vs dinâmica

|             | Estática (array)                 | Dinâmica (Vec)               |
| ----------- | -------------------------------- | ---------------------------- |
| Tamanho     | fixo — definido em `const MAX`   | cresce conforme necessário   |
| Memória     | reservada em tempo de compilação | alocada em tempo de execução |
| Lista cheia | precisa verificar                | impossível de estourar       |

### Gerenciamento de memória — Rust vs outras linguagens

Em C e C++ com alocação dinâmica, o programador precisa alocar e liberar a memória manualmente:

```c
// C — alocação manual
int *lista = malloc(sizeof(int) * MAX);
if (lista == NULL) { return; } // precisa verificar se funcionou
free(lista);                   // precisa liberar no final
```

```cpp
// C++ — alocação manual
int *lista = new int[MAX];
if (lista == nullptr) { return; }
delete[] lista; // precisa liberar no final
```

Em Java a memória é gerenciada pelo **Garbage Collector** — um processo que roda em segundo plano e libera o que não está mais sendo usado. Simples para o programador, mas pode pausar o programa durante a limpeza.

```java
// Java — sem alocação manual, mas com GC em runtime
int[] lista = new int[MAX];
```

Em Rust não tem alocação manual nem Garbage Collector — o compilador determina em tempo de compilação quando cada variável deve ser liberada. Zero custo em execução e zero risco de vazamento de memória.

```rust
// Rust — array estático reservado em tempo de compilação
nos: [None; MAX] // liberado automaticamente quando sai do escopo
```

|                    | C           | C++            | Java               | Rust       |
| ------------------ | ----------- | -------------- | ------------------ | ---------- |
| Aloca manualmente  | sim         | sim (dinâmico) | não                | não        |
| Libera manualmente | sim         | sim (dinâmico) | não                | não        |
| Garbage Collector  | não         | não            | sim                | não        |
| Quem gerencia      | programador | programador    | JVM em runtime     | compilador |
| Custo em execução  | zero        | zero           | sim (pausas do GC) | zero       |

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

A memória é **contígua** — todos os slots ficam lado a lado na memória, endereços sequenciais. A ordem lógica da lista é definida pelos ponteiros `prox`, não pela posição física no array.

### Construtor `new`

```rust
pub fn new() -> Self {
    ListaCircular {
        nos:     [None; MAX], // todos os slots vazios
        prox:    [0; MAX],    // índices zerados
        cabeca:  None,        // sem primeiro nó
        tamanho: 0,           // lista vazia
    }
}
```

`Self` é um atalho para o nome da própria struct. `[None; MAX]` cria um array de MAX posições todas com o valor `None`.

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

**`idx_na`** — caminha `pos` passos a partir da cabeça e retorna o índice físico:

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
    if self.tamanho == MAX { return; }           // 1. lista cheia?

    let slot = self.achar_slot_livre().unwrap(); // 2. acha espaço vazio
    self.nos[slot] = Some(valor);                // 3. guarda o valor

    if self.tamanho == 0 {
        self.prox[slot] = slot;                  // 4. único nó aponta para si
        self.cabeca = Some(slot);
    } else {
        let cabeca = self.cabeca.unwrap();
        let cauda  = self.achar_cauda();
        self.prox[slot] = cabeca;                // 5. novo → antiga cabeça
        self.prox[cauda] = slot;                 //    cauda → novo
        self.cabeca = Some(slot);                //    novo vira a cabeça
    }

    self.tamanho += 1;                           // 6. incrementa o tamanho
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

### Ler próximo

Recebe uma posição e retorna o valor do nó seguinte. Se for a cauda, retorna a cabeça — comportamento circular acontece naturalmente:

```rust
pub fn ler_proximo(&self, pos: usize) -> Option<T> {
    if self.tamanho == 0 || pos >= self.tamanho { return None; }
    let idx = self.idx_na(pos);       // índice físico do nó na posição pos
    self.nos[self.prox[idx]]          // valor do próximo nó
}
```

Exemplo com lista `[5] → [10] → [20] → [30]`:
```
ler_proximo(0) = 10   (próximo do 5)
ler_proximo(3) = 5    (próximo do 30 volta para a cabeça)
```

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

    // descobre o nó a remover (rem) e o anterior (ant)
    let (rem, ant) = if pos == 0 {
        (self.cabeca.unwrap(), self.achar_cauda())
    } else {
        let a = self.idx_na(pos - 1);
        (self.prox[a], a)
    };

    self.prox[ant] = self.prox[rem];                    // anterior pula o removido
    if pos == 0 { self.cabeca = Some(self.prox[rem]); } // atualiza cabeça se necessário

    let valor = self.nos[rem].take();  // libera o slot
    self.tamanho -= 1;
    valor
}
```

O `take()` retira o valor do `Option` e deixa `None` no lugar — libera o slot para uso futuro.

### Percorrer

```rust
pub fn percorrer(&self) {
    if self.tamanho == 0 { println!("lista vazia"); return; }

    let mut idx = self.cabeca.unwrap();
    for i in 0..self.tamanho {      // exatamente N passos — sem loop infinito
        print!("[{}]", self.nos[idx].unwrap());
        if i < self.tamanho - 1 { print!(" → "); }
        idx = self.prox[idx];       // avança para o próximo
    }
    println!(" → (HEAD)");
}
```

O `for` limitado ao `tamanho` é o que impede o loop infinito — sem ele os ponteiros seriam seguidos para sempre.

### Tabela física

Mostra o estado interno dos dois arrays — útil para entender a diferença entre ordem lógica e ordem física:

```rust
pub fn tabela_fisica(&self) {
    println!("  idx │ nos      │ prox");
    println!("  ────┼──────────┼──────");
    for i in 0..MAX {
        let val = match self.nos[i] {
            Some(v) => format!("{:>8}", v),
            None    => "  vazio  ".to_string(),
        };
        let head = if self.cabeca == Some(i) { " ← HEAD" } else { "" };
        println!("   {}  │ {}  │  {}   │{}", i, val, self.prox[i], head);
    }
}
```

Exemplo de saída com lista `[5] → [10] → [20] → [30]`:
```
idx │ nos      │ prox
────┼──────────┼──────
 0  │       10 │  1   │
 1  │       20 │  2   │
 2  │       30 │  3   │
 3  │        5 │  0   │ ← HEAD
 4  │   vazio  │  0   │
```

A ordem física no array é a ordem de inserção. A ordem lógica da lista começa pelo HEAD e segue os ponteiros `prox`.

### Menu interativo

O programa usa um `loop` — equivalente ao `do-while` de outras linguagens:

```rust
loop {
    imprimir_menu();                        // mostra as opções
    io::stdin().read_line(&mut opcao);      // espera o usuário digitar
    let opcao = opcao.trim();               // remove o \n do Enter

    match opcao {
        "1" => lista.inserir_inicio(v),
        "2" => lista.inserir_fim(v),
        "3" => lista.ler_proximo(pos),
        "4" => lista.remover(pos),
        "5" => lista.percorrer(),
        "6" => lista.tabela_fisica(),
        "0" => break,                       // sai do loop
        _   => println!("opção inválida"),  // qualquer outra entrada
    }
}
```

A leitura e conversão do valor digitado é feita diretamente:

```rust
let v: i32    = ler("valor").parse().unwrap();
let pos: usize = ler("posição").parse().unwrap();
```

`parse()` converte a `String` para o tipo anotado. O tipo é inferido pela anotação `: i32` ou `: usize` na variável.

--- 

## 🏋️ Desafio — Implementar o `atualizar`

A lista ainda não tem uma operação de **atualização**. Sua missão é implementar o método abaixo diretamente em `lista_circular_menu.rs`.

### Assinatura

```rust
pub fn atualizar(&mut self, pos: usize, novo_valor: T) -> bool {
    // retorna true se atualizou, false se a posição não existe
    todo!()
}
```

### Esqueleto com dicas

```rust
pub fn atualizar(&mut self, pos: usize, novo_valor: T) -> bool {
    // 1. verifique se a posição é válida
    //    dica: compare pos com self.tamanho

    // 2. ache o índice físico do nó na posição lógica dada
    //    dica: você já tem um auxiliar que faz exatamente isso

    // 3. substitua o valor armazenado naquele slot
    //    dica: self.nos[idx] é um Option<T> — atribua Some(novo_valor)

    // 4. retorne true indicando sucesso
    todo!()
}
```

### O que adicionar no `main`

No `match opcao { ... }` dentro do `loop`, adicione um novo braço:

```rust
"7" => {
    let pos: usize = ler("posição").parse().unwrap();
    let v: i32     = ler("novo valor").parse().unwrap();
    if lista.atualizar(pos, v) {
        println!("posição {} atualizada para {}", pos, v);
    } else {
        println!("posição {} não existe (tamanho: {})", pos, lista.tamanho);
    }
}
```

E adicione a opção ao menu impresso:

```rust
println!("║  7 - Atualizar por posição   ║");
```

### Casos de teste

| Operação           | Lista antes           | Resultado esperado   |
| ------------------ | --------------------- | -------------------- |
| `atualizar(0, 99)` | `[10] → [20] → [30]`  | `[99] → [20] → [30]` |
| `atualizar(2, 99)` | `[10] → [20] → [30]`  | `[10] → [20] → [99]` |
| `atualizar(5, 99)` | lista com 3 elementos | retorna `false`      |
| `atualizar(0, 99)` | lista vazia           | retorna `false`      |

> 💡 **Dica final:** o `atualizar` é o mais simples dos quatro — não mexe nos ponteiros `prox`, não precisa de slot livre, não muda `cabeca` nem `tamanho`. Só localiza e substitui.

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

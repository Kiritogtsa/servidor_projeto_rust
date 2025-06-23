# Thread Pool - Biblioteca Personalizada

🧵 **Implementação de Pool de Threads para Aprendizado**

Esta é uma biblioteca de pool de threads implementada do zero em Rust, criada especificamente para fins de **aprendizado** e como **verificação de conceito (PoC)** dos conceitos de concorrência em Rust.

## 🎯 Objetivos de Aprendizado

**Esta biblioteca demonstra:**
- Implementação de pool de threads sem dependências externas
- Uso de `Arc` e `Mutex` para compartilhamento seguro de dados
- Canais (`mpsc`) para comunicação entre threads
- Gerenciamento do ciclo de vida de threads
- Patterns de ownership e borrowing em contextos concorrentes
- Implementação de `Drop` trait para limpeza adequada de recursos

## 📚 Funcionalidades

- ✅ Criação de pool com número configurável de threads
- ✅ Execução assíncrona de jobs/tarefas
- ✅ Gerenciamento automático do ciclo de vida das threads
- ✅ Finalização limpa de recursos com `Drop`
- ✅ Testes unitários incluídos

## 🚀 Como usar

### Exemplo básico

```rust
use thread_pool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    
    pool.execute(|| {
        println!("Job executado na thread!");
    });
    
    // Pool é automaticamente finalizado quando sai de escopo
}
```

### Exemplo com múltiplos jobs

```rust
use thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    let pool = ThreadPool::new(2);
    let counter = Arc::new(Mutex::new(0));
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Job {}: Counter = {}", i, *num);
        });
    }
    
    // Aguarda jobs terminarem antes de finalizar
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

## 🧪 Testes

Execute os testes com:

```bash
cargo test
```

## 🔧 API

### `ThreadPool::new(size: usize) -> ThreadPool`
Cria um novo pool com o número especificado de threads.

**Panics:** Se `size` for 0.

### `ThreadPool::execute<F>(&self, f: F)`
Executa uma função/closure no pool de threads.

**Onde:** `F: FnOnce() + Send + 'static`

### `ThreadPool::size(&self) -> usize`
Retorna o número de workers no pool.

## 🏗️ Arquitetura Interna

```
ThreadPool
├── workers: Vec<Worker>     // Lista de workers
└── sender: mpsc::Sender     // Canal para enviar jobs

Worker
├── id: usize               // ID único do worker
├── thread: JoinHandle      // Handle da thread
└── receiver: mpsc::Receiver // Recebe jobs do canal compartilhado
```

## 🎓 Conceitos de Rust Demonstrados

- **Arc (Atomic Reference Counting):** Compartilhamento seguro de dados entre threads
- **Mutex:** Exclusão mútua para acesso seguro a dados compartilhados
- **mpsc (Multiple Producer, Single Consumer):** Canais para comunicação entre threads
- **Ownership & Borrowing:** Gerenciamento seguro de memória em contexto concorrente
- **Traits:** Implementação de `Drop` para limpeza de recursos
- **Closures & FnOnce:** Funções como objetos de primeira classe
- **Pattern Matching:** Uso de `match` para tratamento de resultados

## 📁 Estrutura do Projeto

```
thread_pool/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs          # Implementação completa da biblioteca
```

## 🔗 Integração

Esta biblioteca foi projetada para ser usada pelo projeto `servidor`, demonstrando como criar e usar bibliotecas locais em Rust.

**Status:** ✅ Implementação completa e testada


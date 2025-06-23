# Servidor HTTP em Rust

🚀 **Verificação de Conceito (PoC) e Aprendizado**

Este é um projeto de servidor HTTP implementado em Rust que utiliza uma **biblioteca de thread pool personalizada**. Este projeto é uma **verificação de conceito** e foi criado especificamente para **fins de aprendizado** dos fundamentos de programação de rede e concorrência em Rust.

**📦 Dependência:**
- `thread_pool` - Biblioteca personalizada de pool de threads (path: `../thread_pool`)

## Funcionalidades

- Servidor HTTP básico rodando na porta 7878
- Suporte a múltiplas conexões usando threads
- Páginas HTML simples
- Rota principal (`/`) com página de boas-vindas
- Rota de teste (`/sleep`) que simula um delay de 5 segundos
- Tratamento de páginas não encontradas (404)

## Como executar

1. Compile o projeto:
```bash
cargo build
```

2. Execute o servidor:
```bash
cargo run
```

3. Acesse no seu navegador:
- Página principal: http://127.0.0.1:7878
- Teste de delay: http://127.0.0.1:7878/sleep

## Como funciona

O servidor:
1. Escuta conexões TCP na porta 7878
2. Para cada conexão, cria uma nova thread
3. Lê a requisição HTTP
4. Responde com HTML baseado na rota solicitada
5. Suporta as rotas `/` e `/sleep`

## Estrutura do projeto

```
servidor/
├── Cargo.toml          # Configuração do projeto
├── README.md           # Este arquivo
└── src/
    └── main.rs         # Código principal do servidor
```

## Conceitos de Rust demonstrados

- Uso de módulos da biblioteca padrão (`std::net`, etc.)
- **Integração com biblioteca local personalizada** (`thread_pool`)
- Tratamento de erros com `unwrap()`
- Pattern matching com `if` e `match`
- **Pool de threads para concorrência** (via biblioteca personalizada)
- Manipulação de strings e bytes
- TCP sockets
- **Gerenciamento de dependências locais**

## 🎯 Objetivos de Aprendizado

**Este projeto serve como PoC (Proof of Concept) para:**
- Entender programação de rede em Rust
- Praticar conceitos de ownership e borrowing
- Implementar concorrência com threads
- Manipular protocolos de rede (HTTP) sem frameworks
- Aprender sobre sockets TCP/IP
- Gerenciar memória e recursos de forma segura

**Status do desenvolvimento:**
- ✅ Estrutura básica do projeto criada
- 🚧 **EM DESENVOLVIMENTO** - Implementação do servidor sendo feita do zero

Este projeto é ideal para entender os fundamentos de redes e concorrência em Rust sem a complexidade de frameworks externos.

// essa parte não precisa, e depois remove esse comentario daqui, não precisa da chave no readme e nem o lembrete


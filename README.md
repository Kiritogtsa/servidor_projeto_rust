# Projetos de Aprendizado Rust

🦀 **Workspace de Projetos para Estudo de Concorrência e Redes em Rust**

Este é um workspace Cargo que contém projetos de aprendizado focados em conceitos fundamentais de Rust, especialmente concorrência e programação de rede.

## 📁 Estrutura do Workspace

```
projects/
├── Cargo.toml                  # Configuração do workspace
├── README.md                   # Este arquivo
├── servidor/                   # Projeto do servidor HTTP
│   ├── Cargo.toml
│   ├── README.md
│   └── src/main.rs
└── thread_pool/                # Biblioteca de thread pool personalizada
    ├── Cargo.toml
    ├── README.md
    └── src/lib.rs
```

## 🎯 Projetos

### 1. `thread_pool` - Biblioteca de Thread Pool
🧵 **Biblioteca personalizada de pool de threads**

- ✅ Implementação completa de pool de threads
- ✅ Uso de `Arc`, `Mutex` e `mpsc`
- ✅ Gerenciamento automático do ciclo de vida
- ✅ Testes unitários incluídos

**Conceitos demonstrados:**
- Concorrência segura em Rust
- Padrões de ownership e borrowing
- Comunicação entre threads
- Implementação de traits (`Drop`)

### 2. `servidor` - Servidor HTTP
🌐 **Servidor HTTP usando thread pool personalizada**

- 🚧 **Em desenvolvimento** - Implementação do zero
- ✅ Integração com biblioteca `thread_pool`
- 🎯 Objetivo: Servidor HTTP sem frameworks externos

**Conceitos demonstrados:**
- Programação de rede TCP/IP
- Protocolo HTTP básico
- Integração de bibliotecas locais
- Arquitetura cliente-servidor

## 🚀 Como usar

### Executar projeto específico
```bash
# Executar o servidor
cargo run -p servidor

# Executar testes da thread_pool
cargo test -p thread_pool
```

### Executar todo o workspace
```bash
# Compilar todos os projetos
cargo build

# Executar todos os testes
cargo test

# Verificar todos os projetos
cargo check
```

## 🎓 Objetivos de Aprendizado

**Este workspace demonstra:**

1. **Organização de projetos** com Cargo workspace
2. **Bibliotecas personalizadas** e sua integração
3. **Concorrência segura** em Rust
4. **Programação de rede** fundamentais
5. **Padrões de design** em Rust
6. **Testes unitários** e documentação

## 📊 Status dos Projetos

| Projeto | Status | Funcionalidades |
|---------|---------|-----------------|
| `thread_pool` | ✅ Completo | Pool de threads, testes, documentação |
| `servidor` | 🚧 Em desenvolvimento | Estrutura básica, integração thread_pool |

## 🛠️ Próximos Passos

1. **Implementar servidor HTTP completo** no projeto `servidor`
2. **Adicionar mais funcionalidades** ao thread pool (timeouts, métricas)
3. **Expandir testes** e benchmarks
4. **Adicionar logging** e monitoramento

## 📝 Como Contribuir (Aprendizado)

Este é um projeto de aprendizado. Para praticar:

1. Clone o projeto
2. Estude a implementação do `thread_pool`
3. Complete a implementação do `servidor`
4. Experimente com melhorias e otimizações
5. Adicione novos testes e documentação

## 🎯 Verificação de Conceito (PoC)

Ambos os projetos são **verificações de conceito** criadas especificamente para:
- Entender conceitos fundamentais de Rust
- Praticar programação concorrente e de rede
- Demonstrar integração entre projetos Rust
- Servir como referência para estudos futuros


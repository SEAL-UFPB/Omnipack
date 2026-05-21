# Omnipack 🦀

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

O **Omnipack** é um orquestrador e empacotador universal interativo para Linux, escrito em Rust. Ele inspeciona seu diretório, identifica automaticamente a linguagem de programação do projeto através de uma heurística de arquivos âncora e, por meio de uma interface TUI (Terminal User Interface) elegante, gera e testa pacotes nativos e universais de forma totalmente automatizada.

O objetivo do Omnipack é abstrair a complexidade de dezenas de flags e arquivos de configuração do ecossistema Linux (`fpm`, `cpack`, `linuxdeploy`, `flatpak-builder`), centralizando o pipeline de entrega local em uma única ferramenta extremamente rápida e agnóstica.

---

## 🚀 Como Funciona

O fluxo do Omnipack é dividido em quatro etapas executadas localmente:

1. **Auto-Detecção:** Identifica o ecossistema do projeto buscando por assinaturas (ex: `Cargo.toml`, `CMakeLists.txt`, `build.gradle.kts`, `pyproject.toml`).
2. **Interface Interativa (TUI):** Coleta metadados essenciais (nome, versão, descrição) e os formatos de saída desejados por meio de menus de seleção múltipla rápidos.
3. **Build Nativo:** Aciona o compilador ou interpretador correto do projeto no modo otimizado (Release).
4. **Isolamento & Teste de Fumaça (Smoke Test):** Cria diretórios temporários espelhando a árvore de arquivos Linux, invoca os empacotadores especialistas em background e **valida os pacotes gerados iniciando containers descartáveis** via Docker/Podman para garantir que a instalação e a execução funcionaram perfeitamente.

---

## 🛠️ Tecnologias e Dependências do Core

O projeto é construído em Rust focando em máxima performance e segurança de memória:

*   **TUI / Interface:** Baseada em `inquire` para prompts interativos rápidos e estruturados.
*   **Orquestração de Processos:** Gerenciamento nativo através de `std::process::Command` com captura e roteamento de streams (`stdout`/`stderr`).
*   **Validação Automatizada:** Integração com a API do Docker para execução de testes isolados de instalação (`.deb` no Ubuntu, `.rpm` no Fedora, etc.).

> ⚠️ **Pré-requisitos do Sistema:** Para rodar o pipeline completo, o Omnipack assume que as ferramentas de empacotamento (`fpm`, `linuxdeploy`, `flatpak-builder`) e o daemon do Docker/Podman estão instalados ou acessíveis no sistema.

---

## 📂 Arquitetura do Workspace Temporário

Durante a execução, o Omnipack isola os binários compilados em uma árvore estruturada antes de despachá-los para os geradores de pacotes:

```text
/tmp/omnipack-workspace/
└── usr/
    ├── bin/
    │   └── [seu-binario-compilado]
    └── share/
        ├── applications/
        │   └── app.desktop         # Gerado dinamicamente via TUI
        └── icons/
            └── hicolor/64x64/apps/
                └── app.png         # Ícone padrão ou configurado


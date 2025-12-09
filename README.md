# ⚡ PROSPECTOR BTC
### Arquitetura de Auditoria Criptográfica Distribuída em Curva Elíptica secp256k1

![Status](https://img.shields.io/badge/Status-Research_Prototype-blueviolet?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT_Academic-green?style=for-the-badge)
![Core](https://img.shields.io/badge/Core-Rust_SIMD-orange?style=for-the-badge)
![Orchestration](https://img.shields.io/badge/Nx-Monorepo-blue?style=for-the-badge)

---

> **🎓 PROPOSTA DE TESE DE DOUTORADO**
>
> **Autor:** Raz Podesta (MetaShark Tech)
> **Instituição Alvo:** Massachusetts Institute of Technology (MIT) - Cryptography & Systems Security Group
> **Foco:** Cibersegurança Ofensiva, Sistemas Distribuídos e Entropia da Informação.

---

## 🌌 1. Resumo Conceitual: O Arqueólogo de Entropia

O **PROSPECTOR BTC** não é uma ferramenta de força bruta comum; ele é um **Arqueólogo de Entropia Distribuído**.

A segurança da rede Bitcoin baseia-se na impossibilidade termodinâmica de varrer o espaço de chaves de $2^{256}$. No entanto, essa premissa matemática só se sustenta se a geração de entropia for perfeitamente uniforme. Evidências históricas sugerem que chaves geradas por humanos ("Brainwallets") e PRNGs defeituosos (Debian 2008, Android 2013) criaram **aglomerados densos de vulnerabilidade**.

Este projeto implementa a arquitetura **"PROSPECTOR BTC"**: um sistema distribuído de alto desempenho e custo zero, projetado para mapear e auditar esses setores de vulnerabilidade utilizando recursos efêmeros de nuvem.

### 🎯 A Hipótese Central
> *"A barreira de entrada para auditar a resiliência da rede Bitcoin é drasticamente menor do que a teoria dita, se utilizarmos a computação oportunista para atacar falhas humanas em vez da matemática pura."*

---

## 🏛️ 2. Arquitetura Técnica de Elite

O sistema foi projetado como um **Monolito Modular Estrito** gerenciado pelo **Nx**, garantindo fronteiras rígidas entre Domínio, Núcleo Matemático e Infraestrutura.

### O Fluxo de Dados (The Data Pipeline)
```mermaid
graph TD
    subgraph "Nuvem Pública (Fonte)"
        BQ[Google BigQuery] -->|Extração CSV| ETL[App: Census Taker]
    end

    subgraph "Processamento Local (Rust Core)"
        ETL -->|Compilação| BF[Filtro de Bloom (200MB)]
        BF -->|Distribuição| Cloud[Nuvem Fantasma]
    end

    subgraph "The Ghost Cloud (Enxame de Workers)"
        Cloud -->|Carregar Filtro| W1[Colab Worker 01]
        Cloud -->|Carregar Filtro| W2[Colab Worker 02]
        Cloud -->|Carregar Filtro| W3[Colab Worker 300]

        W1 -- SIMD Mining --> Match{Colisão?}
        W2 -- SIMD Mining --> Match
        W3 -- SIMD Mining --> Match
    end

    subgraph "Persistência & Visualização"
        Match -->|SIM| API[Orchestrator API]
        API -->|Store| DB[(Turso DB)]
        DB -->|Query| Web[Next.js Dashboard]
    end
```

🛠️ O Stack Tecnológico
Componente	Tecnologia	Justificativa de Engenharia
Core Engine	Rust (no_std)	Acesso direto à memória e instruções de CPU (AVX-512) para máxima velocidade de hash.
Orquestração	Nx	Gerenciamento de Monorepo com cache computacional para múltiplos binários.
Memória	Bloom Filters	Estruturas probabilísticas O(1) para verificar 50M de endereços em milissegundos.
Infraestrutura	Turso (libSQL)	Banco de dados distribuído na borda (Edge) para persistência de baixo custo.
Poder de Fogo	Google Colab	Utilização de GPUs T4/CPUs de alto desempenho como nós de processamento efêmeros.

📂 3. O Monolito Fractal (Estrutura do Código)
Seguimos o CODEX RAZSMART, garantindo que cada diretório tenha uma responsabilidade única e atômica.

```Text
prospector/
├── apps/                          # 🚀 APLICATIVOS EXECUTÁVEIS
│   ├── orchestrator/              # O Comandante (API Server em Rust/Axum)
│   ├── miner-worker/              # O Soldado (Binário Estático para Linux/Colab)
│   ├── census-taker/              # O Cartógrafo (ETL Rust para BigQuery)
│   └── web-dashboard/             # O Observatório (Next.js 14 Científico)
│
├── libs/                          # 🧩 BLOCOS DE CONSTRUÇÃO (Librerias)
│   ├── core/                      # [CAMADA 1] MATEMÁTICA PURA
│   │   ├── math-engine/           # Curvas Elípticas Otimizadas (secp256k1)
│   │   ├── generators/            # Endereços Legacy, Segwit, WIF
│   │   └── probabilistic/         # Filtros de Bloom Serializáveis
│   │
│   ├── domain/                    # [CAMADA 2] ESTRATÉGIA
│   │   ├── mining-strategy/       # Dicionários de Ataque e Padrões
│   │   └── models/                # Tipos Compartilhados (Rust <-> TS)
│   │
│   └── infra/                     # [CAMADA 3] MUNDO REAL
│       ├── db-turso/              # Conectores SQL
│       └── transport/             # Serialização Zero-Copy
```
🧪 4. Metodologia Científica: "The Ghost Cloud"
Para provar a tese sem custos de infraestrutura proibitivos (AWS/GCP), desenvolvemos a metodologia da "Nuvem Fantasma":
Extração (Extract): Utilizamos o Tier Gratuito do Google BigQuery para reconstruir o set UTXO do Bitcoin sem baixar os 600GB da Blockchain.
Compressão (Compress): Convertemos 50 milhões de endereços ativos em um artefato binário de ~200MB usando Probabilidade Matemática.
Distribuição (Distribute): Implantamos binários Rust estáticos (musl) em 300+ instâncias de notebooks gratuitos (Jupyter/Colab).
Reconciliação (Reconcile): Os nós reportam apenas "sucessos" e "batimentos cardíacos" para a API central, minimizando o tráfego de rede.

📜 5. Licença e Ética
Copyright © 2025 Raz Podesta | MetaShark Tech.
Este projeto é estritamente acadêmico e destinado à pesquisa de segurança. O uso deste software para acessar ativos digitais sem autorização é ilegal. A arquitetura foi desenhada para auditoria de resiliência, não para exploração maliciosa.

Distribuído sob a Licença Acadêmica MIT.

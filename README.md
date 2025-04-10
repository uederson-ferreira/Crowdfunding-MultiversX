Claro, Uederson! Aqui está um `README.md` completo e em português, seguindo fielmente a documentação do projeto de **Crowdfunding** com contratos inteligentes na **MultiversX**, com todos os passos bem explicados para qualquer pessoa conseguir seguir:

---

```markdown
# 📦 Crowdfunding MultiversX

Este projeto é um contrato inteligente escrito em Rust para a rede **MultiversX**, que simula uma campanha de **financiamento coletivo (crowdfunding)**.

Com este contrato, é possível:

- Arrecadar EGLD de diversos usuários;
- Estipular um valor-alvo (meta) e uma data final (prazo);
- Reembolsar os doadores caso a meta não seja atingida;
- Enviar os fundos arrecadados ao dono do contrato se a meta for cumprida.

---

## 🛠 Requisitos

Antes de começar, você precisa ter:

- ✅ [Rust](https://www.rust-lang.org/) versão `>= 1.83.0` instalado
- ✅ `sc-meta` (ferramenta da MultiversX para build e geração de proxies)
- ✅ VS Code com as extensões:
  - `rust-analyzer`
  - `CodeLLDB`

---

## 🚀 Começando do zero

### 1. Criação do projeto

No terminal, execute:

```bash
sc-meta new --name crowdfunding --template empty
```

Isso criará a estrutura inicial do projeto.

---

### 2. Implementando o construtor `init`

No arquivo `src/crowdfunding.rs`, adicione o seguinte:

```rust
#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, target: BigUint) {
        self.target().set(&target);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[view]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;
}
```

---

### 3. Compilando o contrato

Para compilar:

```bash
sc-meta all build
```

Isso irá gerar os arquivos compilados na pasta `output/`.

---

### 4. Criando o Proxy

Crie um arquivo chamado `sc-config.toml` na raiz com:

```toml
[settings]

[[proxy]]
path = "src/crowdfunding_proxy.rs"
```

Gere o proxy:

```bash
sc-meta all proxy
```

Depois, importe o proxy no seu contrato:

```rust
pub mod crowdfunding_proxy;
```

---

### 5. Escrevendo testes de caixa preta (blackbox)

Crie o arquivo `tests/crowdfunding_blackbox_test.rs` com o seguinte conteúdo:

```rust
use crowdfunding::crowdfunding_proxy;
use multiversx_sc_scenario::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");
const OWNER: TestAddress = TestAddress::new("owner");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);

    blockchain
}

#[test]
fn crowdfunding_deploy_test() {
    let mut world = world();

    world.account(OWNER).nonce(0).balance(1_000_000);

    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world.check_account(OWNER).balance(1_000_000);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();
}
```

---

### 6. Executando os testes

Rode o comando:

```bash
cargo test
```

Saída esperada:

```
running 1 test
test crowdfunding_deploy_test ... ok

test result: ok. 1 passed; 0 failed; ...
```

---

## ℹ️ Sobre o contrato

Este contrato é um exemplo básico de como persistir dados com `SingleValueMapper` e executar testes simulando uma blockchain com contas fictícias.  
Ele será expandido para incluir os métodos `fund`, `claim` e `status`.

---

## 📂 Estrutura esperada do projeto

```
crowdfunding/
├── src/
│   ├── crowdfunding.rs
│   └── crowdfunding_proxy.rs
├── tests/
│   └── crowdfunding_blackbox_test.rs
├── output/
│   ├── crowdfunding.wasm
│   ├── crowdfunding.mxsc.json
│   └── ...
├── sc-config.toml
├── Cargo.toml
└── ...
```

---

## 🧠 Conceitos importantes

- **SingleValueMapper**: permite armazenar valores persistentes com chave única.
- **#[init]**: método construtor chamado na implantação.
- **#[view]**: métodos somente leitura.
- **Proxy**: usado para simular chamadas aos endpoints no ambiente de testes.
- **Blackbox testing**: testa o comportamento do contrato simulando a blockchain.

---

## 👨‍💻 Autores

Este projeto segue o [tutorial oficial da MultiversX](https://docs.multiversx.com/developers/tutorials/crowdfunding-p1)  
Tradução e adaptação por Uederson Ferreira.

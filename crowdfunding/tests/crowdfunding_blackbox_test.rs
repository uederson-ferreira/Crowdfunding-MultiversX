// Estamos importando algumas ferramentas que nos ajudam a brincar de blockchain.
use multiversx_sc_scenario::imports::*;
use crowdfunding::crowdfunding_proxy::{self, Status};

// Aqui a gente diz onde está o "código do contrato", quem é o "dono",
// quem é o "doador", e onde o contrato vai ser instalado.
const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");
const OWNER: TestAddress = TestAddress::new("owner");
const DONOR: TestAddress = TestAddress::new("donor");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

// Essa função prepara um mundinho onde vamos testar o contrato.
// É como montar um tabuleiro de brinquedo.
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // Dizemos onde está o contrato dentro da nossa pastinha
    blockchain.set_current_dir_from_workspace("crowdfunding");

    // Registramos o contrato pra poder brincar com ele no mundo simulado
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);

    blockchain
}

// Essa função instala o contrato no nosso mundinho, como se fosse montar o jogo.
// O dono coloca o contrato no lugar, com uma meta e um prazo.
fn crowdfunding_deploy() -> ScenarioWorld {
    let mut world = world();

    // Damos um pouco de dinheiro pro dono
    world.account(OWNER).nonce(0).balance(1_000_000);

    // Agora o dono instala o contrato no mundo com meta e prazo
    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64, 123000u64) // meta: 500 bilhõezinhos, prazo: bloco 123000
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    // Verifica se o contrato foi mesmo colocado no lugar certo
    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world
}

// Esse teste verifica se o contrato foi instalado direitinho
#[test]
fn crowdfunding_deploy_test() {
    let mut world = crowdfunding_fund(); // Reaproveitamos a função que também doa

    // Confirma que o dono ainda tem o dinheiro que demos
    world.check_account(OWNER).nonce(1).balance(1_000_000u64);

    // O doador já doou uma parte, e tem um pouco a menos agora
    world
        .check_account(DONOR)
        .nonce(1)
        .balance(150_000_000_000u64);

    // O contrato está segurando o dinheiro que foi doado
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .nonce(0)
        .balance(250_000_000_000u64);

    // Verifica se a meta do contrato é 500 bilhões
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();

    // Verifica se o prazo está certo
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(123_000u64))
        .run();

    // Verifica se o doador doou mesmo 250 bilhões
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deposit(DONOR)
        .returns(ExpectValue(250_000_000_000u64))
        .run();
}

// Essa função faz o doador doar um pouco de dinheiro pro contrato
fn crowdfunding_fund() -> ScenarioWorld {
    let mut world = crowdfunding_deploy(); // Primeiro, instala o contrato

    // Damos dinheiro pro doador brincar
    world.account(DONOR).nonce(0).balance(400_000_000_000u64);

    // O doador envia 250 bilhões pro contrato
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(250_000_000_000u64)
        .run();

    world
}

// Esse teste verifica se a doação foi feita e tudo ficou certo
#[test]
fn crowdfunding_fund_test() {
    let mut world = crowdfunding_fund();

    world.check_account(OWNER).nonce(1).balance(1_000_000u64);
    world
        .check_account(DONOR)
        .nonce(1)
        .balance(150_000_000_000u64);
    world
        .check_account(CROWDFUNDING_ADDRESS)
        .nonce(0)
        .balance(250_000_000_000u64);

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .target()
        .returns(ExpectValue(500_000_000_000u64))
        .run();
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deadline()
        .returns(ExpectValue(123_000u64))
        .run();
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .deposit(DONOR)
        .returns(ExpectValue(250_000_000_000u64))
        .run();
}

// Este teste verifica o que acontece se tentarmos doar depois do prazo final
#[test]
fn crowdfunding_fund_too_late_test() {
    let mut world = crowdfunding_fund(); // contrato e doação inicial

    // A gente avança o tempo para depois do prazo
    world.current_block().block_timestamp(123_001u64);

    // O doador tenta doar mais dinheiro, mas já é tarde demais
    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(10_000_000_000u64)
        .with_result(ExpectError(4, "cannot fund after deadline")) // Esperamos um erro
        .run();

    // Verifica se o status da campanha virou "falhou"
    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Failed))
        .run();
}

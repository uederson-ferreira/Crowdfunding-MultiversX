//use crowdfunding::crowdfunding_proxy;
use multiversx_sc_scenario::imports::*;
use crowdfunding::crowdfunding_proxy::{self, Status};

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");
const OWNER: TestAddress = TestAddress::new("owner");
const DONOR: TestAddress = TestAddress::new("donor");
const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);
    blockchain
}

// 🔽 ESTA É A FUNÇÃO AUXILIAR PARA IMPLANTAR O CONTRATO
fn crowdfunding_deploy() -> ScenarioWorld {
    let mut world = world();

    world.account(OWNER).nonce(0).balance(1_000_000);

    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64, 123000u64)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(crowdfunding_address, CROWDFUNDING_ADDRESS.to_address());

    world
}

#[test]
fn crowdfunding_deploy_test() {
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

fn crowdfunding_fund() -> ScenarioWorld {
    let mut world = crowdfunding_deploy();

    world.account(DONOR).nonce(0).balance(400_000_000_000u64);

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

#[test]
fn crowdfunding_fund_too_late_test() {
    let mut world = crowdfunding_fund();

    world.current_block().block_timestamp(123_001u64);

    world
        .tx()
        .from(DONOR)
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .fund()
        .egld(10_000_000_000u64)
        .with_result(ExpectError(4, "cannot fund after deadline"))
        .run();

    world
        .query()
        .to(CROWDFUNDING_ADDRESS)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .status()
        .returns(ExpectValue(Status::Failed))
        .run();
}
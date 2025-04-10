use crowdfunding::crowdfunding_proxy;
use multiversx_sc_scenario::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("output/crowdfunding.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("crowdfunding");
    blockchain.register_contract(CODE_PATH, crowdfunding::ContractBuilder);
    blockchain
}

const OWNER: TestAddress = TestAddress::new("owner");

#[test]
fn crowdfunding_deploy_test() {
    let mut world = world();

    world.account(OWNER).nonce(0).balance(1000000);


const CROWDFUNDING_ADDRESS: TestSCAddress = TestSCAddress::new("crowdfunding");
    /*
        Set up account
    */

    let crowdfunding_address = world
        .tx()
        .from(OWNER)
        .typed(crowdfunding_proxy::CrowdfundingProxy)
        .init(500_000_000_000u64)
        .code(CODE_PATH)
        .new_address(CROWDFUNDING_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();
    /*
        Set up account
        Deploy
    */

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
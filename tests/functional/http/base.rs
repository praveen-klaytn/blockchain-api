use {
    super::check_if_rpc_is_responding_correctly_for_supported_chain,
    crate::context::ServerContext,
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
#[ignore]
async fn base_provider_eip155_8453_and_84531(ctx: &mut ServerContext) {
    // Base mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:8453", "0x2105").await;

    // Base Goerli
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:84531", "0x14a33").await
}

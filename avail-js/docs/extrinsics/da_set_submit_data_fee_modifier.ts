import { SDK, WaitFor, Keyring, sdkTransactions } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const modifier = {
    weightMaximumFee: SDK.oneAvail(),
    weightFeeDivider: 20,
  } as sdkTransactions.DispatchFeeModifier

  const result = await sdk.tx.dataAvailability.setSubmitDataFeeModifier(modifier, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()

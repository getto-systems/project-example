import { RemoteNonceGenerator } from "./infra"

export type RemoteOutsideFeature = Readonly<{
    webCrypto: Crypto
}>

export type RemoteFeature = Readonly<{
    serverURL: string
    nonce: RemoteNonceGenerator
}>

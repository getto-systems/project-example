export type ApiFeature = Readonly<{
    serverURL: string
    nonce: ApiNonceGenerator
}>

export type ApiRequest = Readonly<{
    url: string
    options: Readonly<{
        method: ApiMethod
        credentials: "include"
        headers: [[ApiNonceHeader, ApiNonce]]
    }>
}>

export type ApiMethod = "GET" | "POST"

export type ApiNonceHeader = "X-GETTO-EXAMPLE-NONCE"
export type ApiNonce = string

export interface ApiNonceGenerator {
    (): ApiNonce
}

export type RemoteFetchOptions = Readonly<{
    url: string
    options: Readonly<{
        method: RemoteFetchMethod
        credentials: "include"
        headers: RemoteHeader[]
    }>
}>

export type RemoteFetchMethod = "GET" | "POST" | "PATCH" | "PUT" | "DELETE"
export type RemoteHeader = [string, string]

export type RemoteNonce = string

import { RemoteOutsideFeature } from "./feature"

import {
    RemoteFetchMethod,
    RemoteFetchOptions,
    RemoteHeader,
    RemoteNonce,
    RemoteErrorResult,
} from "./infra"

import { RemoteCommonError, RemoteInfraError, RemoteServerError } from "./data"

export type RemoteFetchParams = Readonly<{
    serverURL: string
    path: string
    method: RemoteFetchMethod
    headers: RemoteHeader[]
}>

export function fetchOptions(params: RemoteFetchParams): RemoteFetchOptions {
    return {
        url: url(),
        options: {
            method: params.method,
            credentials: "include",
            headers: params.headers,
        },
    }

    function url(): string {
        const url = new URL(params.serverURL)
        url.pathname = params.path
        return url.toString()
    }
}

export function generateNonce(feature: RemoteOutsideFeature): RemoteNonce {
    const { webCrypto } = feature
    return webCrypto.getRandomValues(new Uint32Array(4)).join("-")
}

export function remoteCommonError(status: number): RemoteErrorResult<RemoteCommonError> {
    return { success: false, err: err() }

    function err(): RemoteCommonError {
        switch (status) {
            case 401:
                return { type: "unauthorized" }

            case 409:
                return { type: "invalid-nonce" }

            default:
                return { type: "server-error" }
        }
    }
}
export function remoteServerError(): RemoteErrorResult<RemoteServerError> {
    return { success: false, err: { type: "server-error" } }
}
export function remoteInfraError(err: unknown): RemoteErrorResult<RemoteInfraError> {
    return { success: false, err: { type: "infra-error", err: `${err}` } }
}

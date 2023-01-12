import { RemoteFetchMethod, RemoteFetchOptions, RemoteHeader, RemoteErrorResult } from "../infra"

import { RemoteCommonError, RemoteInfraError, RemoteServerError } from "../data"

export type RemoteFetchParams = Readonly<{
    serverURL: string
    path: string
    method: RemoteFetchMethod
    headers?: readonly RemoteHeader[]
}>

export function fetchOptions(params: RemoteFetchParams): RemoteFetchOptions {
    return {
        url: url(),
        options: {
            method: params.method,
            credentials: "include",
            headers: Array.from(params.headers || []),
        },
    }

    function url(): string {
        const url = new URL(params.serverURL)
        url.pathname = params.path
        return url.toString()
    }
}

export function remoteCommonError(status: number): RemoteErrorResult<RemoteCommonError> {
    return { success: false, err: err() }

    function err(): RemoteCommonError {
        switch (status) {
            case 401:
                return { type: "unauthorized" }

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

import { RemoteOutsideFeature } from "./feature"

import {
    Remote,
    RemoteFetchMethod,
    RemoteFetchOptions,
    RemoteHeader,
    RemoteNonce,
    RemotePod,
} from "./infra"

import { RemoteCommonError, RemoteErrorResult, RemoteInfraError, RemoteServerError } from "./data"

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
    return feature.webCrypto.getRandomValues(new Uint32Array(4)).join("-")
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

export function convertRemote<M, V, R, E_raw, E_unknown>(
    remote: Remote<M, R, E_raw>,
): RemotePod<M, V, R, E_raw | E_unknown> {
    return (converter) => async (message) => {
        const result = await remote(message)
        if (!result.success) {
            return result
        }
        return { success: true, value: converter(result.value) }
    }
}

export function passThroughRemoteValue<T>(value: T): T {
    return value
}

export type RemoteCommonErrorReason = Readonly<{
    message: string
    detail: string[]
}>
export function remoteCommonErrorReason<T>(
    err: RemoteCommonError,
    message: { (reason: RemoteCommonErrorReason): T[] },
): T[] {
    switch (err.type) {
        case "unauthorized":
            return message({
                message: "認証エラー",
                detail: ["もう一度ログインしてください"],
            })

        case "invalid-nonce":
            return message({
                message: "接続エラー",
                detail: [
                    "もう一度操作してください",
                    "繰り返しエラーになる場合、お手数ですが管理者に連絡お願いします",
                ],
            })

        case "server-error":
            return message({
                message: "サーバーエラー",
                detail: ["お手数ですが管理者に連絡お願いします"],
            })

        case "infra-error":
            return message({ message: "ネットワークエラー", detail: detail(err.err) })
    }

    function detail(message: string): string[] {
        if (message.length === 0) {
            return []
        }
        return [`(詳細: ${message})`]
    }
}

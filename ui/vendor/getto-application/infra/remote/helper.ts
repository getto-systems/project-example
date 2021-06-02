import { RemoteCommonError } from "./data"
import { Remote, RemoteFeature, RemoteOutsideFeature, RemotePod } from "./infra"

export function remoteFeature(
    serverURL: string,
    { webCrypto }: RemoteOutsideFeature,
): RemoteFeature {
    return {
        serverURL,
        nonce: () => webCrypto.getRandomValues(new Uint32Array(4)).join("-"),
    }
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
export function remoteCommonError<T>(
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

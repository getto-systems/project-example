import { RemoteCommonError } from "./data"

export type RemoteCommonErrorReason = Readonly<{
    message: string
    detail: readonly string[]
}>
export function remoteCommonErrorReason<T>(
    err: RemoteCommonError,
    message: { (reason: RemoteCommonErrorReason): readonly T[] },
): readonly T[] {
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

    function detail(message: string): readonly string[] {
        if (message.length === 0) {
            return []
        }
        return [`(詳細: ${message})`]
    }
}

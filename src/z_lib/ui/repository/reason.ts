import { RepositoryError } from "./data"

export type RepositoryErrorReason = Readonly<{
    message: string
    detail: readonly string[]
}>
export function repositoryErrorReason<T>(
    err: RepositoryError,
    message: { (reason: RepositoryErrorReason): readonly T[] },
): readonly T[] {
    switch (err.type) {
        case "infra-error":
            return message({ message: "データベースエラー", detail: detail(err.err) })
    }

    function detail(message: string): readonly string[] {
        if (message.length === 0) {
            return []
        }
        return [`(詳細: ${message})`]
    }
}

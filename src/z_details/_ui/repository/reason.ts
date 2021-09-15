import { RepositoryError } from "./data"

export type RepositoryErrorReason = Readonly<{
    message: string
    detail: string[]
}>
export function repositoryErrorReason<T>(
    err: RepositoryError,
    message: { (reason: RepositoryErrorReason): T[] },
): T[] {
    switch (err.type) {
        case "infra-error":
            return message({ message: "データベースエラー", detail: detail(err.err) })
    }

    function detail(message: string): string[] {
        if (message.length === 0) {
            return []
        }
        return [`(詳細: ${message})`]
    }
}

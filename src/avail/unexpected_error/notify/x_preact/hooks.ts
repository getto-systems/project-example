import { useErrorBoundary } from "preact/hooks"

import { NotifyUnexpectedErrorAction } from "../action"

export function useNotifyUnexpectedError({
    error,
}: Readonly<{ error: NotifyUnexpectedErrorAction }>): unknown {
    const [err] = useErrorBoundary((err) => error.notify(err))
    return err
}

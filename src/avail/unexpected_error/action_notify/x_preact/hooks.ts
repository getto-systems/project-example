import { useErrorBoundary } from "preact/hooks"

import { NotifyUnexpectedErrorResource } from "../resource"

export function useNotifyUnexpectedError({ error }: NotifyUnexpectedErrorResource): unknown {
    const [err] = useErrorBoundary((err) => error.notify(err))
    return err
}

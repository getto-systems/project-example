import { VNode } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"
import { html } from "htm/preact"

import { SelectFileAction } from "../action"

import { FileStoreConnector } from "../infra"

type Props = Readonly<{
    file: SelectFileAction
}> &
    Partial<{
        disabled: boolean
    }>
export function SelectFileComponent({ file, disabled }: Props): VNode {
    return html`<input
        ref=${useFileRef(file.connector)}
        type="file"
        onInput=${onInput}
        disabled=${disabled}
    />`

    function onInput() {
        file.publisher.post()
    }
}

function useFileRef(connector: FileStoreConnector) {
    const REF = useRef<HTMLInputElement>()

    useLayoutEffect(() => {
        connector.connect({
            get: () => {
                if (!REF.current) {
                    return { found: false }
                }
                const files = REF.current.files
                if (files === null) {
                    return { found: false }
                }
                if (files.length === 0) {
                    return { found: false }
                }
                return { found: true, file: files[0] }
            },
        })
        return () => connector.terminate()
    }, [connector])

    return REF
}

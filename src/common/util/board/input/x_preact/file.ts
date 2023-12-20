import { html } from "htm/preact"
import { PreactNode } from "../../../../x_preact/vnode"

import { useLayoutEffect, useRef } from "preact/hooks"

import { InputBoardAction } from "../action"

import { BoardStoreConnector, FileBoardStore } from "../infra"

type Props = Readonly<{
    file: InputBoardAction<FileBoardStore>
}> &
    Partial<{
        disabled: boolean
    }>
export function SelectFile({ file, disabled }: Props): PreactNode {
    return html`<input
        ref=${useFileRef(file.connector)}
        type="file"
        onInput=${onInput}
        disabled=${disabled}
    />`

    function onInput() {
        file.onInput()
    }
}

function useFileRef(connector: BoardStoreConnector<FileBoardStore>) {
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

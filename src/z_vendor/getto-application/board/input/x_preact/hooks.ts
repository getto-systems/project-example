import { Ref } from "preact"
import { useLayoutEffect, useRef } from "preact/hooks"

import { readBoardValue } from "../../kernel/convert"

import { BoardValueStore, BoardValueStoreConnector } from "../infra"

export function useInputRef(
    connector: BoardValueStoreConnector<BoardValueStore>,
): Ref<HTMLInputElement | undefined> {
    const REF = useRef<HTMLInputElement>()

    useLayoutEffect(() => {
        connector.connect({
            get: () => {
                if (!REF.current) {
                    return ""
                }
                return readBoardValue(REF.current)
            },
            set: (value) => {
                if (REF.current) {
                    REF.current.value = value
                }
            },
        })
        return () => connector.terminate()
    }, [connector])

    return REF
}

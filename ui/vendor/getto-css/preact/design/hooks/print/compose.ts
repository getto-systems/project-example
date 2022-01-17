import { useEffect, useState } from "preact/hooks"

import { VNodeKey } from "../../../common"

// rows をページの高さによって分割する
// design/print の report で出力される構造で、
// rows が report のコンテンツの table - tbody - tr として整形される場合に使用できる
export function useReportRowsComposition<R>(
    rows: readonly R[],
    content: ReportRowsCompositionContent
): ReportRowsComposition<R> {
    const [data, setData] = useState(initialReportRowsComposition(rows))
    useEffect(() => {
        const nextComposition = composeReportRows(content, data)
        if (nextComposition.hasNext) {
            setData(nextComposition.data)
        }
    }, [content, data])
    return data
}

type ReportRowsComposition<R> = Readonly<{
    pagedRows: readonly ReadonlyArray<R>[]
    composeIndex: number
}>

type ReportRowsCompositionContent = Readonly<{
    root: { (index: number): HTMLElement | null }
    rowKey: { (tr: HTMLTableRowElement): string | null }
}>

function initialReportRowsComposition<R>(rows: readonly R[]): ReportRowsComposition<R> {
    return { pagedRows: [rows], composeIndex: 0 }
}

function composeReportRows<R>(
    { rowKey, root }: ReportRowsCompositionContent,
    data: ReportRowsComposition<R>
): NextReportRowsComposition<R> {
    type ComposeInfo = Readonly<{ changeKeyCount: number; currentKey: CurrentKey }>
    type CurrentKey =
        | Readonly<{ type: "initial" }>
        | Readonly<{ type: "focused"; key: VNodeKey }>
        | Readonly<{ type: "notFound" }>

    type ContentHeight = Readonly<{ found: false }> | Readonly<{ found: true; height: number }>

    const page = root(data.composeIndex)
    if (!page) {
        return { hasNext: false }
    }

    const contentHeight = findContentHeight(page)
    if (!contentHeight.found) {
        return { hasNext: false }
    }

    return findNextComposition(page, contentHeight.height)

    function findContentHeight(page: HTMLElement): ContentHeight {
        // report__contentLimit__mark は css によって report の padding 内部の高さになっている
        const markers = page.getElementsByClassName("report__contentLimit__mark")
        for (const marker of markers) {
            if (marker instanceof HTMLElement) {
                return {
                    found: true,
                    // padding 内部の高さから report__header の高さを除いたものを基準とすることで
                    // ヘッダに追加要素があっても適切に高さを計算できるようにする
                    height: marker.offsetHeight - reportHeaderHeight() - reportFooterHeight(),
                }
            }
        }
        return { found: false }

        function reportHeaderHeight() {
            const elements = page.getElementsByClassName("report__header")
            for (const element of elements) {
                if (element instanceof HTMLElement) {
                    return element.offsetHeight
                }
            }
            return 0
        }
        function reportFooterHeight() {
            const elements = page.getElementsByClassName("report__footer")
            for (const element of elements) {
                if (element instanceof HTMLElement) {
                    return element.offsetHeight
                }
            }
            return 0
        }
    }

    function findNextComposition(page: HTMLElement, height: number): NextReportRowsComposition<R> {
        let info = initialComposeInfo()

        const tableBodies = page.getElementsByTagName("tbody")
        for (const body of tableBodies) {
            const tableRows = body.getElementsByTagName("tr")
            for (const tr of tableRows) {
                info = nextComposeInfo(info, rowKey(tr))

                if (body.offsetTop + tr.offsetTop + tr.offsetHeight > height) {
                    return nextComposition(data, info.changeKeyCount)
                }
            }
        }

        return { hasNext: false }
    }

    function initialComposeInfo(): ComposeInfo {
        return { changeKeyCount: 0, currentKey: { type: "initial" } }
    }
    function nextComposeInfo(info: ComposeInfo, key: string | null): ComposeInfo {
        if (!key) {
            return { changeKeyCount: info.changeKeyCount, currentKey: { type: "notFound" } }
        }

        switch (info.currentKey.type) {
            case "initial":
                return {
                    changeKeyCount: info.changeKeyCount,
                    currentKey: { type: "focused", key },
                }

            case "notFound":
                return {
                    changeKeyCount: info.changeKeyCount + 1,
                    currentKey: { type: "focused", key },
                }

            case "focused":
                if (key === info.currentKey.key) {
                    return info
                } else {
                    return {
                        changeKeyCount: info.changeKeyCount + 1,
                        currentKey: { type: "focused", key },
                    }
                }
        }
    }
}

type NextReportRowsComposition<R> =
    | Readonly<{ hasNext: false }>
    | Readonly<{ hasNext: true; data: ReportRowsComposition<R> }>

function nextComposition<R>(
    data: ReportRowsComposition<R>,
    splitIndex: number
): NextReportRowsComposition<R> {
    return {
        hasNext: true,
        data: {
            pagedRows: [
                ...data.pagedRows.slice(0, -1),
                ...splitRows(data.pagedRows[data.pagedRows.length - 1]),
            ],
            composeIndex: data.composeIndex + 1,
        },
    }

    function splitRows<R>(rows: readonly R[]): readonly ReadonlyArray<R>[] {
        // スペースによっては splitIndex に 0 が指定される場合がある
        // 少なくとも 1行は分割しないと無限ループになるのでその場合は index に 1 を使用する
        const index = Math.max(1, splitIndex)
        return [rows.slice(0, index), rows.slice(index)]
    }
}

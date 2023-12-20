import { PreactContent, PreactNode } from "../../../../common/x_preact/vnode"
import { html } from "htm/preact"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { label_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"

import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../action"

import { RepositoryError } from "../../../../common/util/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
}>
export function DisplaySeason(props: Props): PreactNode {
    const state = useAtom(props.season.state)

    return info(body())

    function body(): PreactContent {
        switch (state.type) {
            case "initial":
                return html``

            case "success":
                return seasonLabel(state.season)

            case "failed":
                return errorContent(state.err)
        }
    }
}

function info(body: PreactContent): PreactNode {
    return html`<small>シーズン:</small> ${body}`
}

function errorContent(err: RepositoryError): readonly PreactNode[] {
    return [label_alert("ロードエラー"), ...detail()]

    function detail(): readonly PreactNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [html` <small>詳細: ${err.err}</small>`]
    }
}

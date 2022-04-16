import { VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { label_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
}>
export function LoadSeason(props: Props): VNode {
    const state = useApplicationAction(props.season)

    return info(body())

    function body(): VNodeContent {
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

function info(body: VNodeContent) {
    return html`<small>シーズン:</small> ${body}`
}

function errorContent(err: RepositoryError) {
    return [label_alert("ロードエラー"), ...detail()]

    function detail(): VNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [html` <small>詳細: ${err.err}</small>`]
    }
}

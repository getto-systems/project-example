import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { label_alert } from "../../../../../ui/vendor/getto-css/preact/design/highlight"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { LoadSeasonResource, LoadSeasonResourceState } from "../resource"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { seasonLabel } from "../../kernel/helper"

export function LoadSeasonEntry({ season }: LoadSeasonResource): VNode {
    return h(LoadSeasonComponent, { season, state: useApplicationAction(season) })
}

type Props = LoadSeasonResource & LoadSeasonResourceState
export function LoadSeasonComponent(props: Props): VNode {
    return info(body())

    function body(): VNodeContent {
        switch (props.state.type) {
            case "initial-season":
                return EMPTY_CONTENT

            case "succeed-to-load":
                return seasonLabel(props.state.season)

            case "failed-to-load":
                return errorContent(props.state.err)
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

const EMPTY_CONTENT = html``

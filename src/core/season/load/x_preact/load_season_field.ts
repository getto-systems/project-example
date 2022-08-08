import { VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { field } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
}>
export function LoadSeasonField(props: Props): VNode {
    const state = useApplicationState(props.season.state)

    return field({
        title: "シーズン",
        body: body(),
    })

    function body(): VNodeContent {
        switch (state.type) {
            case "initial":
                return html``

            case "success":
                return seasonLabel(state.season)

            case "failed":
                return loadError(state.err)
        }
    }
}

function loadError(err: RepositoryError): VNodeContent {
    return [notice_alert("ロードエラー"), ...detail()]

    function detail(): VNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [v_small(), html`<small><p>詳細: ${err.err}</p></small>`]
    }
}

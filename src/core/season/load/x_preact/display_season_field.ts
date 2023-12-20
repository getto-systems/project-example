import { PreactContent, PreactNode } from "../../../../common/x_preact/node"
import { html } from "htm/preact"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { field } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../action"

import { RepositoryError } from "../../../../common/util/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
}>
export function DisplaySeasonField(props: Props): PreactNode {
    const state = useAtom(props.season.state)

    return field({
        title: "シーズン",
        body: body(),
    })

    function body(): PreactContent {
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

function loadError(err: RepositoryError): PreactContent {
    return [notice_alert("ロードエラー"), ...detail()]

    function detail(): readonly PreactNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [v_small(), html`<small><p>詳細: ${err.err}</p></small>`]
    }
}

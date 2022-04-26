import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../z_vendor/getto-css/preact/design/box"
import { field, fieldHelp_error } from "../../../../z_vendor/getto-css/preact/design/form"

import { InputSeason } from "../../input/x_preact/input"
import { EditButton } from "../../../../z_lib/ui/button/edit_button"
import { ChangeButton } from "../../../../z_lib/ui/button/change_button"

import { repositoryErrorReason } from "../../../../z_lib/ui/repository/x_error/reason"
import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../../load/action"
import { SetupSeasonAction } from "../action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
    setup: Readonly<{
        season: SetupSeasonAction
        editable: EditableBoardAction
    }>
}>
export function SetupSeason(props: Props): VNode {
    const state = useApplicationAction(props.setup.season)
    const editableState = useApplicationAction(props.setup.editable)
    const loadState = useApplicationAction(props.season)

    switch (loadState.type) {
        case "initial":
        case "failed":
            return html``

        case "success":
            return box({
                title: "シーズン設定",
                ...(editableState.isEditable
                    ? {
                          body: [
                              h(InputSeason, {
                                  title: "シーズン",
                                  field: props.setup.season.season,
                                  seasons: loadState.availableSeasons,
                              }),
                          ],
                          footer: [setupButton(), ...message()],
                          form: true,
                      }
                    : {
                          body: [
                              field({
                                  title: "シーズン",
                                  body: seasonLabel(loadState.season),
                              }),
                          ],
                          footer: editButton(),
                      }),
            })
    }

    function editButton(): VNode {
        return h(EditButton, { isSuccess: state.type === "success", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.setup.editable.open()
        }
    }

    function setupButton(): VNode {
        // select による選択なので validate しない; validateState は "initial" 固定
        return h(ChangeButton, { isConnecting: false, validateState: "initial", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.setup.season.setup().then((state) => {
                if (state.type === "success") {
                    props.setup.editable.close()
                }
            })
        }
    }

    function message(): VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
                return []

            case "invalid":
                return [fieldHelp_error(["シーズンの設定に失敗しました"])]

            case "failed":
                return [fieldHelp_error(repositoryError(state.err))]
        }
    }
}

function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりシーズン設定に失敗しました`,
        ...reason.detail,
    ])
}

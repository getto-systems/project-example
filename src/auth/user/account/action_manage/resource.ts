import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { SearchUserAccountAction } from "../action_search/action"

export type ManageUserAccountView = ApplicationView<ManageUserAccountResource>

export type ManageUserAccountResource = Readonly<{
    search: SearchUserAccountAction
}>

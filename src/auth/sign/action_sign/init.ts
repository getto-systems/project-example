import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"

import { initialSignViewState, SignAction, SignActionState, SignSubView } from "./action"

import { SignViewDetecter, SignViewType } from "../router/data"
import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

export function initSignAction(detecter: SignViewDetecter, subView: SignSubView): SignAction {
    return new Action(detecter, subView)
}

class Action extends ApplicationAbstractStateAction<SignActionState> implements SignAction {
    readonly initialState = initialSignViewState

    subView: SignSubView

    constructor(detecter: SignViewDetecter, subView: SignSubView) {
        super(async () => {
            const view = this.subView.check()
            const viewType = detecter()

            view.resource.subscriber.subscribe((state) => {
                switch (state.type) {
                    case "required-to-login":
                        this.post(this.mapViewType(viewType))
                        return
                }
            })

            if (viewType.valid) {
                switch (viewType.value) {
                    case "password-reset":
                        return this.post(this.mapViewType(viewType))
                }
            }
            return this.post({ type: "check-authTicket", view: view })
        })
        this.subView = subView
    }

    async error(err: string): Promise<SignActionState> {
        return this.post({ type: "error", err })
    }

    mapViewType(result: ConvertLocationResult<SignViewType>): SignActionState {
        if (!result.valid) {
            // 特に指定が無ければパスワードログイン
            return {
                type: "password-authenticate",
                view: this.subView.password_authenticate(),
            }
        }

        const type = result.value
        switch (type) {
            case "static-privacyPolicy":
                return { type, resource: this.subView.link() }

            case "password-reset-requestToken":
                return { type, view: this.subView.password_reset_requestToken() }
            case "password-reset":
                return { type, view: this.subView.password_reset() }
        }
    }
}

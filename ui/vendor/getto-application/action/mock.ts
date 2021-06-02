import { ApplicationAbstractStateAction } from "./impl"

import { ApplicationStateAction } from "./action"
import { ApplicationActionIgniteHook } from "./infra"

export abstract class ApplicationMockStateAction<S>
    extends ApplicationAbstractStateAction<S>
    implements ApplicationStateAction<S> {
    constructor(hook: ApplicationActionIgniteHook<S> = async () => this.initialState) {
        super(async () => {
            const state = await hook()
            return this.initialState === state ? state : this.post(state)
        })
    }
}

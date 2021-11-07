import { ObserveBoardFieldInfra } from "./infra"

import { ObserveBoardFieldResult } from "./data"

export interface CheckBoardFieldMethod {
    (post: Post<ObserveBoardFieldResult>): void
}

interface Check {
    (infra: ObserveBoardFieldInfra): CheckBoardFieldMethod
}
export const checkBoardField: Check = (infra) => (post) => {
    const { observer } = infra
    post({ hasChanged: observer.hasChanged() })
}

interface Post<E> {
    (event: E): void
}

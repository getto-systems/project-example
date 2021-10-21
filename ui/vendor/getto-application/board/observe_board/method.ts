import { ObserveBoardStateFound, ObserveBoardStore } from "./infra"

import { ObserveBoardFieldResult } from "../observe_field/data"

export interface UpdateObserveBoardStateMethod<N extends string> {
    (name: N, hasChanged: boolean, post: Post<ObserveBoardFieldResult>): void
}

interface Update {
    <N extends string>(
        fields: readonly N[],
        store: ObserveBoardStore,
    ): UpdateObserveBoardStateMethod<N>
}
export const updateObserveBoardState: Update = (fields, infra) => (name, hasChanged, post) => {
    const { stack } = infra

    stack.set(name, hasChanged)
    post(compose(fields.map((field) => stack.get(field))))
}

function compose(results: ObserveBoardStateFound[]): ObserveBoardFieldResult {
    if (results.some((result) => result.found && result.hasChanged)) {
        return { hasChanged: true }
    }
    return { hasChanged: false }
}

interface Post<E> {
    (event: E): void
}

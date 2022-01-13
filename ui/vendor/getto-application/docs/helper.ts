import {
    DocsAction_legacy,
    DocsActionContent_legacy,
    DocsActionTargetType,
    DocsAction_action,
    DocsAction_request,
    DocsContent,
    DocsDescription,
    DocsNegativeNote,
    DocsSection,
    DocsDataDescription,
    DocsData,
    DocsDomain,
    DocsUsecase,
} from "./data"

export function docsUsecase(docs: DocsDomain, path: string): DocsUsecase {
    const usecase = docs.usecase.filter((usecase) => usecase.path === path)
    if (usecase.length === 0) {
        throw new Error(
            `usecase not found: ${path} (${docs.usecase.map((usecase) => usecase.path)})`,
        )
    }
    return usecase[0]
}

export function docsData(title: string, data: readonly DocsDataDescription[]): DocsData {
    return { title, data }
}

// TODO 以下削除予定

export function docsSection(title: string, body: DocsContent[]): DocsSection {
    return { type: "normal", title, body }
}
export function docsSection_pending(title: string, body: DocsContent[]): DocsSection {
    return { type: "pending", title, body }
}
export function docsSection_double(title: string, body: DocsContent[]): DocsSection {
    return { type: "double", title, body }
}

export function docsPurpose(content: string[]): DocsContent {
    return { type: "purpose", content }
}
export function docsModule(content: string[]): DocsContent {
    return { type: "module", content }
}
export function docsItem(title: string, content: string[]): DocsContent {
    return { type: "item", title, content }
}
export function docsDescription(content: DocsDescription[]): DocsContent {
    return { type: "description", content }
}
export function docsExplanation(target: DocsActionTargetType[]): DocsContent {
    return { type: "explanation", target }
}
export function docsNegativeNote(content: DocsNegativeNote[]): DocsContent {
    return { type: "negativeNote", content }
}
export function docsAction_legacy(content: {
    (factory: DocsActionFactory): DocsAction_legacy[]
}): DocsContent {
    return { type: "action", content: content({ request, action, message, validate }) }

    function request(content: DocsAction_request): DocsAction_legacy {
        return { type: "request", content }
    }
    function action(content: DocsAction_action): DocsAction_legacy {
        return { type: "action", content }
    }
    function message(messages: string[]): DocsActionContent_legacy[] {
        return messages.map((message) => ({ type: "normal", message }))
    }
    function validate(messages: string[]): DocsActionContent_legacy[] {
        return messages.map((message) => ({ type: "validate", message }))
    }
}
export function docsNote(content: string[]): DocsContent {
    return { type: "note", content }
}

export interface DocsActionFactory {
    request(content: DocsAction_request): DocsAction_legacy
    action(content: DocsAction_action): DocsAction_legacy
    message(messages: string[]): DocsActionContent_legacy[]
    validate(messages: string[]): DocsActionContent_legacy[]
}

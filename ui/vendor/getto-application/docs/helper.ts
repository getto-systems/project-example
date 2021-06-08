import {
    DocsAction_legacy,
    DocsActionContent,
    DocsActionTargetType,
    DocsAction_action,
    DocsAction_request,
    DocsUsecase,
    DocsContent,
    DocsDescription,
    DocsDomain,
    DocsNegativeNote,
    DocsSection,
    DocsAction,
    DocsActionItem,
    DocsActionItemType,
    DocsDataDescription,
    DocsUsecaseDescriptionMap,
    DocsUsecaseMap,
    DocsUsecaseDescription,
    DocsDomainContent,
    DocsUsecaseContent,
    DocsData,
} from "./data"

export function docsDomainContent<U, A, D>(domain: DocsDomain<U, A, D>): DocsDomainContent {
    return {
        title: domain.title,
        purpose: domain.purpose,
        usecase: domain.usecase.map((name) => toUsecaseContent(domain.toUsecase(name))),
    }
}
export function docsUsecaseContent<U, A, D>(
    domain: DocsDomain<U, A, D>,
    usecase: U,
): DocsUsecaseContent[] {
    return domain.usecase
        .filter((name) => name === usecase)
        .map((name) => toUsecaseContent(domain.toUsecase(name)))
}
function toUsecaseContent<A, D>(usecase: DocsUsecase<A, D>): DocsUsecaseContent {
    return {
        title: usecase.toAction(usecase.title).title,
        purpose: usecase.purpose,
        action: usecase.action.map(usecase.toAction),
        data: usecase.data.map(usecase.toData),
    }
}

export function docsDomain<U, A, D>(
    title: string,
    purpose: string[],
    usecase: U[],
    toUsecase: DocsUsecaseMap<U, A, D>,
): DocsDomain<U, A, D> {
    return { title, purpose, usecase, toUsecase }
}
export function docsUsecase<A, D>(
    title: A,
    purpose: string[],
    content: DocsUsecaseDescription<A, D>,
    map: DocsUsecaseDescriptionMap<A, D>,
): DocsUsecase<A, D> {
    return { title, purpose, ...content, ...map }
}
export function docsAction(
    title: string,
    item: {
        (builder: { item: DocsActionItemBuilder }): DocsActionItem[]
    },
): DocsAction {
    return {
        title,
        item: item({
            item: docsActionItem,
        }),
    }
}
function docsActionItem(
    type: DocsActionItemType,
    content: string[],
    help: string[] = [],
): DocsActionItem {
    return { type, content, help }
}

export interface DocsActionItemBuilder {
    (type: DocsActionItemType, content: string[]): DocsActionItem
    (type: DocsActionItemType, content: string[], help: string[]): DocsActionItem
}

export function docsData(
    title: string,
    data: {
        (builder: Readonly<{ data: DocsDataDescriptionBuilder }>): DocsDataDescription[]
    },
): DocsData {
    return {
        title,
        data: data({
            data: docsDataDescription,
        }),
    }
}
function docsDataDescription(description: string, help: string[] = []): DocsDataDescription {
    return { description, help }
}

export interface DocsDataDescriptionBuilder {
    (description: string): DocsDataDescription
    (description: string, help: string[]): DocsDataDescription
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
    function message(messages: string[]): DocsActionContent[] {
        return messages.map((message) => ({ type: "normal", message }))
    }
    function validate(messages: string[]): DocsActionContent[] {
        return messages.map((message) => ({ type: "validate", message }))
    }
}
export function docsNote(content: string[]): DocsContent {
    return { type: "note", content }
}

export interface DocsActionFactory {
    request(content: DocsAction_request): DocsAction_legacy
    action(content: DocsAction_action): DocsAction_legacy
    message(messages: string[]): DocsActionContent[]
    validate(messages: string[]): DocsActionContent[]
}

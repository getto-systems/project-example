import { DocsDataDescription, DocsData, DocsDomain, DocsUsecase } from "./data"

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

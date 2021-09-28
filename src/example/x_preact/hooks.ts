import { useEffect } from "preact/hooks"

export function useDocumentTitle(title: string): void {
    useEffect(() => {
        document.title = `${title} | ${document.title}`
    }, [title])
}

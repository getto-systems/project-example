import { SearchSort, SearchSortOrder } from "./data"

export function nextSort(currentSort: SearchSort, key: string): SearchSort {
    return { key, order: order() }

    function order(): SearchSortOrder {
        if (key !== currentSort.key) {
            return "normal"
        }
        switch (currentSort.order) {
            case "normal":
                return "reverse"
            case "reverse":
                return "normal"
        }
    }
}

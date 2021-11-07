import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { LoadBreadcrumbListComponent } from "./load_breadcrumb_list"

import { mockBreadcrumbList, mockLoadBreadcrumbListAction } from "../mock"

const options = ["home", "empty"] as const

export default {
    title: "library/Outline/Menu/Load Breadcrumb List",
    argTypes: {
        load: {
            control: { type: "select", options },
        },
    },
}

type MockProps = Readonly<{
    load: typeof options[number]
    label: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(LoadBreadcrumbListComponent, {
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList(props.label)),
    })
})

export const LoadBreadcrumbList = template({ load: "home", label: "ホーム" })

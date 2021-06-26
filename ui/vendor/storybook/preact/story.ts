import { VNode } from "preact"

export function storyTemplate<P>(story: Story<P>): { (props: P): unknown } {
    return (props) => {
        const template = story.bind({})
        template.args = props
        return template
    }
}
export interface Story<P> {
    args?: P
    (args: P): VNode
}

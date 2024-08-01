# Nav

```rust demo

view! {
    <NavDrawer>
        <NavCategory value="area">
            <NavCategoryItem slot icon=icondata::AiAreaChartOutlined>
                "Area Chart"
            </NavCategoryItem>
            <NavSubItem value="target">
                "Target"
            </NavSubItem>
            <NavSubItem value="above">
                "Above"
            </NavSubItem>
            <NavSubItem value="below">
                "Below"
            </NavSubItem>
        </NavCategory>
        <NavCategory value="pie">
            <NavCategoryItem slot icon=icondata::AiPieChartOutlined>
                "Pie Chart"
            </NavCategoryItem>
            <NavSubItem value="pie-target">
                "Pie Target"
            </NavSubItem>
            <NavSubItem value="pin-above">
                "Pin Above"
            </NavSubItem>
            <NavSubItem value="pin-below">
                "Pin Below"
            </NavSubItem>
        </NavCategory>
        <NavItem
            icon=icondata::AiGithubOutlined
            value="github"
            href="https://github.com/thaw-ui/thaw"
            attr:target="_blank"
        >
            "Github"
        </NavItem>
        <NavItem icon=icondata::AiChromeOutlined value="chrome">
            "Chrome"
        </NavItem>
    </NavDrawer>
}
```

# Nav

```rust demo

view! {
    <NavDrawer>
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
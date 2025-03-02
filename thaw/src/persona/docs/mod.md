# Persona

```rust demo
view! {
    <Persona
        name="Thaw"
        avatar_src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"
    >
        <PersonaSecondaryText slot>
            "UI"
        </PersonaSecondaryText>
    </Persona>
}
```

### Text Alignment

A Persona supports two text alignments, `start` being the default position.

```rust demo
view! {
    <Flex justify=FlexJustify::SpaceAround>
        <Persona
            name="Thaw UI"
            text_alignment=PersonaTextAlignment::Start
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
            <PersonaTertiaryText slot>
                "Card"
            </PersonaTertiaryText>
            <PersonaQuaternaryText slot>
                "Header"
            </PersonaQuaternaryText>
        </Persona>
        <Persona
            name="Thaw UI"
            text_alignment=PersonaTextAlignment::Center
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
            <PersonaTertiaryText slot>
                "Card"
            </PersonaTertiaryText>
            <PersonaQuaternaryText slot>
                "Header"
            </PersonaQuaternaryText>
        </Persona>
    </Flex>
}
```

### Text Position

A Persona supports three text positions, `after` being the default position.

```rust demo
view! {
    <Flex justify=FlexJustify::SpaceAround>
        <Persona
            name="Thaw"
            text_position=PersonaTextPosition::After
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            text_position=PersonaTextPosition::Below
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            text_position=PersonaTextPosition::Before
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
    </Flex>
}
```

### Avatar Size

A Persona supports different sizes, medium being the default.

```rust demo
view! {
    <Flex>
        <Persona
            name="Thaw"
            size=PersonaSize::ExtraSmall
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            size=PersonaSize::Small
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            size=PersonaSize::Medium
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            size=PersonaSize::Large
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            size=PersonaSize::ExtraLarge
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
        <Persona
            name="Thaw"
            size=PersonaSize::Huge
        >
            <PersonaSecondaryText slot>
                "UI"
            </PersonaSecondaryText>
        </Persona>
    </Flex>
}
```

### Persona Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| style | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `None` | The name of the person or entity represented by the Persona. |
| size | `Signal<PersonaSize>` | `PersonaSize::Medium` | The size of a Persona and its text. |
| text_alignment | `Signal<PersonaTextAlignment>` | `PersonaTextAlignment::Start` | The vertical alignment of the text relative to the avatar. |
| text_position | `Signal<PersonaTextPosition>` | `PersonaTextPosition::After` | The position of the text relative to the avatar. |
| persona_primary_text | slot `Option<PersonaPrimaryText>` | `None` | The first line of text in the Persona, larger than the rest of the lines. |
| persona_secondary_text | slot `Option<PersonaSecondaryText>` | `None` | The second line of text in the Persona. |
| persona_tertiary_text | slot `Option<PersonaTertiaryText>` | `None` | The third line of text in the Persona. |
| persona_quaternary_text | slot `Option<PersonaQuaternaryText>` | `None` | The fourth line of text in the Persona. |

### PersonaPrimaryText & PersonaSecondaryText & PersonaTertiaryText & PersonaQuaternaryText Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

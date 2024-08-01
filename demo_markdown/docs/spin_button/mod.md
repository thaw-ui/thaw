# SpinButton

```rust demo
let value = RwSignal::new(0);
let value_f64 = RwSignal::new(0.0);

view! {
    <Space vertical=true>
        <SpinButton value step_page=1/>
        <SpinButton value=value_f64 step_page=1.2/>
    </Space>
}
```

### Min / Max

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton value step_page=1 min=-1 max=2/>
}
```

### Disabled

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton value step_page=1 disabled=true/>
}
```

### Custom parsing

```rust demo
let value = RwSignal::new(0.0);

let format = move |v: f64| {
    let v = v.to_string();
    let dot_pos = v.chars().position(|c| c == '.').unwrap_or_else(|| v.chars().count());
    let mut int: String = v.chars().take(dot_pos).collect();

    let sign: String = if v.chars().take(1).collect::<String>() == String::from("-") { 
        int = int.chars().skip(1).collect();
        String::from("-")
    } else { 
        String::from("") 
    };

    let dec: String = v.chars().skip(dot_pos + 1).take(2).collect();

    let int = int
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");
    format!("{}{},{:0<2}", sign, int, dec)
};

let parser = move |v: String| {
    let comma_pos = v.chars().position(|c| c == ',').unwrap_or_else(|| v.chars().count());
    let int_part = v.chars().take(comma_pos).filter(|a| a.is_digit(10)).collect::<String>();
    let dec_part = v.chars().skip(comma_pos + 1).take(2).filter(|a| a.is_digit(10)).collect::<String>();
    
    format!("{:0<1}.{:0<2}", int_part, dec_part).parse::<f64>().ok()
};

view! {
    <SpinButton value parser format step_page=1.0 />
    <p>"Underlying value: "{ value }</p>
}
```
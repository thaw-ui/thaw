# Upload

```rust demo
let message = use_message();
let custom_request = move |file_list: FileList| {
    message.create(
        format!("Number of uploaded files: {}", file_list.length()),
        MessageVariant::Success,
        Default::default(),
    );
};

view!{
    <Upload>
        <Button>
            "upload"
        </Button>
    </Upload>
}
```

### Drag to upload

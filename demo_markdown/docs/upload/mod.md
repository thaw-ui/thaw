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

```rust demo
let message = use_message();
let custom_request = move |file_list: FileList| {
    message.create(
        format!("Number of uploaded files: {}", file_list.length()),
        MessageVariant::Success,
        Default::default(),
    );
};

view! {
    <Upload custom_request>
        <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
    </Upload>
}
```

### Upload Props
